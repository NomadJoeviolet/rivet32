//! STM32F407IGH6 UFBGA176 reference configuration.
use super::common::{self, ImuBus};
use embodied_framework::stm32::{can::CanAdapter, hal, uart::UartRx};
use hal::{
    bind_interrupts, can, dma,
    gpio::{Input, Level, Output, OutputType, Speed},
    peripherals, spi,
    time::Hertz,
    timer::simple_pwm::{PwmPin, SimplePwm},
    usart, usb,
};
use static_cell::StaticCell;
pub const NAME: &str = "STM32F407IGH6";
bind_interrupts!(pub struct Irqs {
    CAN1_TX => can::TxInterruptHandler<peripherals::CAN1>;
    CAN1_RX0 => can::Rx0InterruptHandler<peripherals::CAN1>;
    CAN1_RX1 => can::Rx1InterruptHandler<peripherals::CAN1>;
    CAN1_SCE => can::SceInterruptHandler<peripherals::CAN1>;
    CAN2_TX => can::TxInterruptHandler<peripherals::CAN2>;
    CAN2_RX0 => can::Rx0InterruptHandler<peripherals::CAN2>;
    CAN2_RX1 => can::Rx1InterruptHandler<peripherals::CAN2>;
    CAN2_SCE => can::SceInterruptHandler<peripherals::CAN2>;
    USART1 => usart::InterruptHandler<peripherals::USART1>;
    DMA2_STREAM5 => dma::InterruptHandler<peripherals::DMA2_CH5>;
    OTG_FS => usb::InterruptHandler<peripherals::USB_OTG_FS>;
});
static USB_EP: StaticCell<[u8; 256]> = StaticCell::new();
pub fn clock_config() -> hal::Config {
    common::f4_clocks()
}
/// USART1 pins/DMA are reserved without a selected serial format. Preserve ownership
/// and require an application-supplied Config instead of guessing its baud rate.
pub struct UnconfiguredUart1 {
    pub peripheral: hal::Peri<'static, peripherals::USART1>,
    pub rx: hal::Peri<'static, peripherals::PB7>,
    pub tx: hal::Peri<'static, peripherals::PA9>,
    pub rx_dma: hal::Peri<'static, peripherals::DMA2_CH5>,
}
impl UnconfiguredUart1 {
    pub fn into_receiver(
        self,
        config: usart::Config,
    ) -> Result<UartRx<'static>, usart::ConfigError> {
        usart::UartRx::new(self.peripheral, self.rx, self.rx_dma, Irqs, config).map(UartRx::new)
    }
}
pub struct Resources {
    pub can: [CanAdapter<'static>; 2],
    pub imu: ImuBus,
    pub uart1: UnconfiguredUart1,
    pub motor_pwm: SimplePwm<'static, peripherals::TIM1>,
    pub auxiliary_pwm: SimplePwm<'static, peripherals::TIM4>,
    pub usb: usb::Driver<'static, peripherals::USB_OTG_FS>,
    pub reserved: [Input<'static>; 12],
}
impl Resources {
    pub fn new(p: hal::Peripherals) -> Self {
        let can = common::bxcan_pair_loopback(
            can::Can::new(p.CAN1, p.PD0, p.PD1, Irqs),
            can::Can::new(p.CAN2, p.PB5, p.PB6, Irqs),
            9,
            4,
        );
        let mut config = spi::Config::default();
        config.frequency = Hertz(328_125);
        config.mode = spi::MODE_3;
        let imu = ImuBus {
            spi: spi::Spi::new_blocking(p.SPI1, p.PB3, p.PA7, p.PB4, config),
            accelerometer_cs: Output::new(p.PA4, Level::High, Speed::Low),
            gyroscope_cs: Output::new(p.PB0, Level::High, Speed::Low),
        };
        let uart1 = UnconfiguredUart1 {
            peripheral: p.USART1,
            rx: p.PB7,
            tx: p.PA9,
            rx_dma: p.DMA2_CH5,
        };
        let mut motor_pwm = SimplePwm::new(
            p.TIM1,
            Some(PwmPin::new(p.PE9, OutputType::PushPull)),
            Some(PwmPin::new(p.PE11, OutputType::PushPull)),
            Some(PwmPin::new(p.PE13, OutputType::PushPull)),
            Some(PwmPin::new(p.PE14, OutputType::PushPull)),
            Hertz(500),
            Default::default(),
        );
        motor_pwm.ch1().disable();
        motor_pwm.ch2().disable();
        motor_pwm.ch3().disable();
        motor_pwm.ch4().disable();
        let mut auxiliary_pwm = SimplePwm::new(
            p.TIM4,
            None,
            None,
            Some(PwmPin::new(p.PD14, OutputType::PushPull)),
            None,
            Hertz(4000),
            Default::default(),
        );
        auxiliary_pwm.ch3().disable();
        let mut config = usb::Config::default();
        config.vbus_detection = false;
        let usb = usb::Driver::new_fs(
            p.USB_OTG_FS,
            p.PA12,
            p.PA11,
            Irqs,
            USB_EP.init([0; 256]),
            config,
        );
        let pins: [hal::Peri<'static, hal::gpio::AnyPin>; 12] = [
            p.PB14.into(),
            p.PB15.into(),
            p.PC4.into(),
            p.PC5.into(),
            p.PC6.into(),
            p.PC8.into(),
            p.PF6.into(),
            p.PG0.into(),
            p.PH10.into(),
            p.PH11.into(),
            p.PH12.into(),
            p.PI6.into(),
        ];
        Self {
            can,
            imu,
            uart1,
            motor_pwm,
            auxiliary_pwm,
            usb,
            reserved: pins.map(common::reserve_input),
        }
    }
}
