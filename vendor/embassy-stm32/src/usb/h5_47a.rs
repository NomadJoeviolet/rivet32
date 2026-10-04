//! H5E/F integration checks around the existing Synopsys OTG device driver.
use embassy_usb_synopsys_otg::otg_v1::Otg;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Error {
    FullSpeedClock,
    HighSpeedReference,
    PhyNotInitialized,
    CoreIdentity,
}

pub(super) fn validate_clock(
    high_speed: bool,
    frequency: u32,
    phy_output: Option<u32>,
) -> Result<(), Error> {
    if high_speed {
        if phy_output != Some(48_000_000) {
            return Err(Error::PhyNotInitialized);
        }
        if ![
            16_000_000, 19_200_000, 20_000_000, 24_000_000, 26_000_000, 32_000_000,
        ]
        .contains(&frequency)
        {
            return Err(Error::HighSpeedReference);
        }
    } else if frequency.abs_diff(48_000_000) > 120_000 {
        return Err(Error::FullSpeedClock);
    }
    Ok(())
}

pub(super) fn enable_transceiver(registers: Otg, high_speed: bool) -> Result<(), Error> {
    let expected = if high_speed { 0x6200 } else { 0x5100 };
    if registers.cid().read().0 != expected {
        return Err(Error::CoreIdentity);
    }
    if !high_speed {
        // ST USB_CoreInit enables the FS transceiver at GCCFG bit 16.
        // On the HS core that position controls charger detection instead.
        registers.gccfg_v2().modify(|w| w.set_pwrdwn(true));
    }
    Ok(())
}

pub(super) fn configure_device(registers: Otg, high_speed: bool) -> Result<(), Error> {
    let expected = if high_speed { 0x6200 } else { 0x5100 };
    if registers.cid().read().0 != expected {
        return Err(Error::CoreIdentity);
    }

    // Exact H5E/F USB_CoreInit + USB_SetCurrentMode register operations from
    // STM32CubeH5 HAL ba20038d. These cores have fixed internal PHY wiring;
    // this path needs neither GHWCFG4 nor the generic PHYIF/ULPI selectors.
    registers.gusbcfg().modify(|w| {
        if high_speed {
            w.set_tsdps(false);
        } else {
            w.set_physel(true);
        }
        w.set_fhmod(false);
        w.set_fdmod(true);
    });
    while registers.gintsts().read().cmod() {}
    Ok(())
}
