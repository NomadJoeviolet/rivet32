//! STM32G431KBU6 reference configuration; see the documented pin assignments.
use super::common::{self, ImuBus};
use embodied_framework::stm32::{can::CanAdapter, hal, uart::UartRx};
use hal::{
    bind_interrupts, can, dma,
    gpio::{Input, Level, Output, Speed},
    peripherals, spi,
    time::Hertz,
    usart,
};
pub const NAME: &str = "STM32G431KBU6";
bind_interrupts!(pub struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
    USART2 => usart::InterruptHandler<peripherals::USART2>;
    DMA1_CHANNEL1 => dma::InterruptHandler<peripherals::DMA1_CH1>;
    DMA1_CHANNEL4 => dma::InterruptHandler<peripherals::DMA1_CH4>;
    FDCAN1_IT0 => can::IT0InterruptHandler<peripherals::FDCAN1>;
    FDCAN1_IT1 => can::IT1InterruptHandler<peripherals::FDCAN1>;
});
pub fn clock_config() -> hal::Config {
    common::g4_clocks()
}
pub struct Resources {
    pub can: CanAdapter<'static>,
    pub receive: [UartRx<'static>; 2],
    pub imu: ImuBus,
    /// USART TX, RS485 DE, and PA8: not driven by this reference image.
    pub reserved: [Input<'static>; 4],
}
impl Resources {
    pub fn new(p: hal::Peripherals) -> Self {
        let can = common::fdcan_loopback(
            can::CanConfigurator::new(p.FDCAN1, p.PA11, p.PA12, Irqs),
            (17, 6, 3, 1),
            (2, 10, 6, 1),
        );
        let receive = [
            UartRx::new(
                usart::UartRx::new(
                    p.USART1,
                    p.PB7,
                    p.DMA1_CH1,
                    Irqs,
                    common::uart_config(115_200),
                )
                .unwrap(),
            ),
            UartRx::new(
                usart::UartRx::new(
                    p.USART2,
                    p.PA3,
                    p.DMA1_CH4,
                    Irqs,
                    common::uart_config(10_000_000),
                )
                .unwrap(),
            ),
        ];
        let mut config = spi::Config::default();
        // Original /16 is 10.625MHz. /32 respects BMI088's 10MHz maximum.
        config.frequency = Hertz(5_312_500);
        let imu = ImuBus {
            spi: spi::Spi::new_blocking(p.SPI1, p.PA5, p.PA7, p.PA6, config),
            accelerometer_cs: Output::new(p.PB0, Level::High, Speed::Low),
            gyroscope_cs: Output::new(p.PA4, Level::High, Speed::Low),
        };
        let pins: [hal::Peri<'static, hal::gpio::AnyPin>; 4] =
            [p.PB6.into(), p.PA2.into(), p.PA1.into(), p.PA8.into()];
        Self {
            can,
            receive,
            imu,
            reserved: pins.map(common::reserve_input),
        }
    }
}
