//! DIE47A USB HS PHY clock validation and initialization.
use crate::pac::{pwr, rcc, syscfg};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Source {
    Hse,
    HseDiv2,
    Pll1QDiv2,
    Pll3Q,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Error {
    MissingClock,
    NotHseDerived,
    NonIntegralDivision,
    UnsupportedFrequency,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Reference {
    pub frequency: u32,
    pub fsel: u8,
}

pub(super) struct Registers {
    pub rcc: rcc::Rcc,
    pub pwr: pwr::Pwr,
    pub sbs: syscfg::Sbs,
}

pub(super) fn initialize(
    enabled: bool,
    registers: Registers,
    source: Source,
    hse: Option<u32>,
    pll1_q: Option<(u32, bool)>,
    pll3_q: Option<(u32, bool)>,
) -> Result<Option<u32>, Error> {
    if !enabled {
        return Ok(None);
    }
    let reference = reference(source, hse, pll1_q, pll3_q)?;
    Ok(Some(configure(
        registers.rcc,
        registers.pwr,
        registers.sbs,
        reference,
    )))
}

pub(super) fn reference(
    source: Source,
    hse: Option<u32>,
    pll1_q: Option<(u32, bool)>,
    pll3_q: Option<(u32, bool)>,
) -> Result<Reference, Error> {
    // HSE, including an accurate external clock in bypass mode, is required.
    // PLL1/PLL3 sourced from HSI or CSI do not meet this PHY clock contract.
    let hse = hse.ok_or(Error::MissingClock)?;
    let (frequency, hse_derived, divisor) = match source {
        Source::Hse => (hse, true, 1),
        Source::HseDiv2 => (hse, true, 2),
        Source::Pll1QDiv2 => {
            let (frequency, hse_derived) = pll1_q.ok_or(Error::MissingClock)?;
            (frequency, hse_derived, 2)
        }
        Source::Pll3Q => {
            let (frequency, hse_derived) = pll3_q.ok_or(Error::MissingClock)?;
            (frequency, hse_derived, 1)
        }
    };
    if !hse_derived {
        return Err(Error::NotHseDerived);
    }
    if frequency % divisor != 0 {
        return Err(Error::NonIntegralDivision);
    }
    let frequency = frequency / divisor;
    // Exact RCC_OTGPHYREFCKCLKSOURCE_* encodings from the locked ST HAL.
    let fsel = match frequency {
        16_000_000 => 3,
        19_200_000 => 8,
        20_000_000 => 9,
        24_000_000 => 10,
        26_000_000 => 14,
        32_000_000 => 11,
        _ => return Err(Error::UnsupportedFrequency),
    };
    Ok(Reference { frequency, fsel })
}

pub(super) fn configure(
    rcc: rcc::Rcc,
    pwr: pwr::Pwr,
    sbs: syscfg::Sbs,
    reference: Reference,
) -> u32 {
    // Follow the exact H5E5/H5F5 Cube MSP sequence. Wait for VDDUSB before
    // removing its isolation, as required by HAL_PWREx_EnableVddUSB.
    pwr.usbscr().modify(|w| w.set_usb33den(true));
    while !pwr.vmsr().read().usb33rdy() {}
    pwr.usbscr().modify(|w| w.set_usb33sv(true));
    rcc.ahb2enr().modify(|w| {
        w.set_otghsen(true);
        w.set_otgphyen(true);
    });
    // ST RCC clock-enable macros use a readback for the peripheral bus delay.
    let _ = rcc.ahb2enr().read();
    rcc.ccipr4()
        .modify(|w| w.set_otgphyrefcksel(rcc::vals::Otgphyrefcksel::from_bits(reference.fsel)));
    sbs.otghsphytuner2().modify(|w| {
        w.set_compdistune(2);
        w.set_sqrxtune(0);
    });
    pwr.usbscr().modify(|w| w.set_otghsen(true));
    // Nominal output after source validation and PHY initialization. H5E/F
    // exposes no dedicated PHY PLL-ready flag. This is not a clock measurement.
    // RCC owns this PHY for the configured-clock lifetime so FS may use CLK48
    // independently of the HS controller's device-bus lifetime.
    48_000_000
}
