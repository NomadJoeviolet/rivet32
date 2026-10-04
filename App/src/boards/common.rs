use embodied_framework::stm32::hal;
use hal::{gpio::Output, mode::Blocking, spi};

pub type ImuSpi = spi::Spi<'static, Blocking, spi::mode::Master>;

/// One physical SPI bus, two independent inactive-high chip selects. No bus or
/// CS is duplicated. The application chooses a shared-bus policy before passing
/// the two devices to BMI088; the existing SpiBusDevice adapter supports borrows.
pub struct ImuBus {
    pub spi: ImuSpi,
    pub accelerometer_cs: Output<'static>,
    pub gyroscope_cs: Output<'static>,
}

pub fn uart_config(baudrate: u32) -> hal::usart::Config {
    let mut config = hal::usart::Config::default();
    config.baudrate = baudrate;
    config
}

#[cfg(any(feature = "board-stm32f407ig", feature = "board-stm32f427ii"))]
pub fn f4_clocks() -> hal::Config {
    use hal::rcc::*;
    use hal::time::Hertz;
    let mut config = hal::Config::default();
    config.rcc.hse = Some(Hse {
        freq: Hertz(12_000_000),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll_src = PllSource::Hse;
    config.rcc.pll = Some(Pll {
        prediv: PllPreDiv::Div6,
        mul: PllMul::Mul168,
        divp: Some(PllPDiv::Div2),
        divq: Some(PllQDiv::Div7),
        divr: None,
    });
    config.rcc.sys = Sysclk::Pll1P;
    config.rcc.ahb_pre = AHBPrescaler::Div1;
    config.rcc.apb1_pre = APBPrescaler::Div4;
    config.rcc.apb2_pre = APBPrescaler::Div2;
    config
}

#[cfg(any(feature = "board-stm32g431kb", feature = "board-stm32g474ce"))]
pub fn g4_clocks() -> hal::Config {
    use hal::rcc::*;
    use hal::time::Hertz;
    let mut config = hal::Config::default();
    config.rcc.hse = Some(Hse {
        freq: Hertz(12_000_000),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll = Some(Pll {
        source: PllSource::Hse,
        prediv: PllPreDiv::Div3,
        mul: PllMul::Mul85,
        divp: Some(PllPDiv::Div2),
        divq: Some(if cfg!(feature = "board-stm32g431kb") {
            PllQDiv::Div2
        } else {
            PllQDiv::Div4
        }),
        divr: Some(PllRDiv::Div2),
    });
    config.rcc.sys = Sysclk::Pll1R;
    config.rcc.boost = true;
    config.rcc.mux.fdcansel = mux::Fdcansel::Pclk1;
    // HSI48 is the USB source; the 85MHz PLLQ is not a USB clock.
    config.rcc.hsi48 = Some(Hsi48Config {
        sync_from_usb: false,
    });
    config.rcc.mux.clk48sel = mux::Clk48sel::Hsi48;
    config
}

/// Disconnect unknown actuator GPIOs rather than infer their active polarity.
pub fn reserve_input(pin: hal::Peri<'static, impl hal::gpio::Pin>) -> hal::gpio::Input<'static> {
    hal::gpio::Input::new(pin, hal::gpio::Pull::None)
}

#[cfg(any(
    feature = "board-stm32h723vg",
    feature = "board-stm32g431kb",
    feature = "board-stm32g474ce"
))]
pub fn fdcan_loopback(
    mut can: hal::can::CanConfigurator<'static>,
    nominal: (u16, u8, u8, u8),
    data: (u16, u8, u8, u8),
) -> embodied_framework::stm32::can::CanAdapter<'static> {
    use hal::can::config::{FrameTransmissionConfig, NominalBitTiming};
    let mut config = can.config();
    config.nbtr = NominalBitTiming {
        prescaler: nominal.0.try_into().unwrap(),
        seg1: nominal.1.try_into().unwrap(),
        seg2: nominal.2.try_into().unwrap(),
        sync_jump_width: nominal.3.try_into().unwrap(),
    };
    config.dbtr.prescaler = data.0.try_into().unwrap();
    config.dbtr.seg1 = data.1.try_into().unwrap();
    config.dbtr.seg2 = data.2.try_into().unwrap();
    config.dbtr.sync_jump_width = data.3.try_into().unwrap();
    config.frame_transmit = FrameTransmissionConfig::AllowFdCanAndBRS;
    config.automatic_retransmit = false;
    can.set_config(config);
    embodied_framework::stm32::can::CanAdapter::start(
        can,
        hal::can::OperatingMode::InternalLoopbackMode,
    )
}

#[cfg(any(feature = "board-stm32f407ig", feature = "board-stm32f427ii"))]
pub fn bxcan_pair_loopback(
    mut master: hal::can::Can<'static>,
    slave: hal::can::Can<'static>,
    seg1: u8,
    seg2: u8,
) -> [embodied_framework::stm32::can::CanAdapter<'static>; 2] {
    // CAN2 has no filter RAM of its own. Configure both halves through CAN1;
    // reset-default disabled banks would otherwise discard every loopback RX.
    {
        use hal::can::{Fifo, filter::Mask32};
        let mut filters = master.modify_filters();
        filters.set_split(14).clear();
        filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
        filters
            .slave_filters()
            .clear()
            .enable_bank(14, Fifo::Fifo0, Mask32::accept_all());
    }
    [
        bxcan_loopback(master, seg1, seg2),
        bxcan_loopback(slave, seg1, seg2),
    ]
}

#[cfg(any(feature = "board-stm32f407ig", feature = "board-stm32f427ii"))]
pub fn bxcan_loopback(
    mut can: hal::can::Can<'static>,
    seg1: u8,
    seg2: u8,
) -> embodied_framework::stm32::can::CanAdapter<'static> {
    can.modify_config()
        .set_bit_timing(hal::can::util::NominalBitTiming {
            prescaler: 3_u16.try_into().unwrap(),
            seg1: seg1.try_into().unwrap(),
            seg2: seg2.try_into().unwrap(),
            sync_jump_width: 1_u8.try_into().unwrap(),
        })
        .set_loopback(true)
        .set_silent(true)
        .set_automatic_retransmit(false);
    can.blocking_enable();
    embodied_framework::stm32::can::CanAdapter::new(can)
}
