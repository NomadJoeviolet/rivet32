//! STM32G474CET6 reference configuration. Unresolved TIM2
//! mappings remain inputs; see docs/reference-boards.md for wiring.
use super::common::{self, ImuBus, ImuSpi};
use embodied_framework::stm32::{can::CanAdapter, hal, uart::UartRx};
use hal::{
    bind_interrupts, can, dma,
    gpio::{Input, Level, Output, OutputType, Speed},
    peripherals, spi,
    time::Hertz,
    timer::simple_pwm::{PwmPin, SimplePwm},
    usart, usb,
};
pub const NAME: &str = "STM32G474CET6";
bind_interrupts!(pub struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
    USART2 => usart::InterruptHandler<peripherals::USART2>;
    DMA1_CHANNEL2 => dma::InterruptHandler<peripherals::DMA1_CH2>;
    DMA1_CHANNEL3 => dma::InterruptHandler<peripherals::DMA1_CH3>;
    FDCAN1_IT0 => can::IT0InterruptHandler<peripherals::FDCAN1>;
    FDCAN1_IT1 => can::IT1InterruptHandler<peripherals::FDCAN1>;
    FDCAN2_IT0 => can::IT0InterruptHandler<peripherals::FDCAN2>;
    FDCAN2_IT1 => can::IT1InterruptHandler<peripherals::FDCAN2>;
    FDCAN3_IT0 => can::IT0InterruptHandler<peripherals::FDCAN3>;
    FDCAN3_IT1 => can::IT1InterruptHandler<peripherals::FDCAN3>;
    USB_LP => usb::InterruptHandler<peripherals::USB>;
});
pub fn clock_config() -> hal::Config {
    common::g4_clocks()
}
pub struct Resources {
    pub can: [CanAdapter<'static>; 3],
    pub receive: [UartRx<'static>; 2],
    pub imu: ImuBus,
    pub encoder_spi: ImuSpi,
    pub encoder_cs: [Output<'static>; 2],
    pub servo_pwm: SimplePwm<'static, peripherals::TIM3>,
    pub usb: usb::Driver<'static, peripherals::USB>,
    pub reserved: [Input<'static>; 7],
}
impl Resources {
    pub fn new(p: hal::Peripherals) -> Self {
        let nominal = (17, 6, 3, 1);
        let data = (2, 10, 6, 1);
        let can = [
            common::fdcan_loopback(
                can::CanConfigurator::new(p.FDCAN1, p.PB8, p.PB9, Irqs),
                nominal,
                data,
            ),
            common::fdcan_loopback(
                can::CanConfigurator::new(p.FDCAN2, p.PB5, p.PB6, Irqs),
                nominal,
                data,
            ),
            common::fdcan_loopback(
                can::CanConfigurator::new(p.FDCAN3, p.PA8, p.PB4, Irqs),
                nominal,
                data,
            ),
        ];
        let receive = [
            UartRx::new(
                usart::UartRx::new(
                    p.USART1,
                    p.PA10,
                    p.DMA1_CH2,
                    Irqs,
                    common::uart_config(2_000_000),
                )
                .unwrap(),
            ),
            UartRx::new(
                usart::UartRx::new(
                    p.USART2,
                    p.PA3,
                    p.DMA1_CH3,
                    Irqs,
                    common::uart_config(921_600),
                )
                .unwrap(),
            ),
        ];
        let mut config = spi::Config::default();
        config.frequency = Hertz(5_312_500);
        let imu = ImuBus {
            spi: spi::Spi::new_blocking(p.SPI2, p.PB13, p.PB15, p.PB14, config),
            accelerometer_cs: Output::new(p.PB12, Level::High, Speed::Low),
            gyroscope_cs: Output::new(p.PB11, Level::High, Speed::Low),
        };
        config.frequency = Hertz(10_625_000); // Encoder SPI1 retains source /16.
        let encoder_spi = spi::Spi::new_blocking(p.SPI1, p.PA5, p.PA7, p.PA6, config);
        let encoder_cs = [
            Output::new(p.PB1, Level::High, Speed::Low),
            Output::new(p.PB2, Level::High, Speed::Low),
        ];
        let mut servo_pwm = SimplePwm::new(
            p.TIM3,
            None,
            Some(PwmPin::new(p.PA4, OutputType::PushPull)),
            None,
            None,
            Hertz(50),
            Default::default(),
        );
        servo_pwm.ch2().disable();
        let usb = usb::Driver::new(p.USB, p.PA12, p.PA11, Irqs);
        let pins: [hal::Peri<'static, hal::gpio::AnyPin>; 7] = [
            p.PA9.into(),
            p.PA2.into(),
            p.PA15.into(),
            p.PB3.into(),
            p.PC13.into(),
            p.PC14.into(),
            p.PC15.into(),
        ];
        Self {
            can,
            receive,
            imu,
            encoder_spi,
            encoder_cs,
            servo_pwm,
            usb,
            reserved: pins.map(common::reserve_input),
        }
    }
}
