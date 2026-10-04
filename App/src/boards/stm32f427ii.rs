//! STM32F427IIH6 reference configuration without an onboard IMU. Actuator GPIOs remain inputs.
use super::common;
use embodied_framework::stm32::{can::CanAdapter, hal, uart::UartRx};
use hal::{
    bind_interrupts, can, dma,
    gpio::{Input, OutputType},
    peripherals,
    time::Hertz,
    timer::simple_pwm::{PwmPin, SimplePwm},
    usart, usb,
};
use static_cell::StaticCell;
pub const NAME: &str = "STM32F427IIH6";
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
    USART6 => usart::InterruptHandler<peripherals::USART6>;
    UART8 => usart::InterruptHandler<peripherals::UART8>;
    UART7 => usart::InterruptHandler<peripherals::UART7>;
    DMA2_STREAM2 => dma::InterruptHandler<peripherals::DMA2_CH2>;
    DMA2_STREAM1 => dma::InterruptHandler<peripherals::DMA2_CH1>;
    DMA1_STREAM6 => dma::InterruptHandler<peripherals::DMA1_CH6>;
    DMA1_STREAM3 => dma::InterruptHandler<peripherals::DMA1_CH3>;
    OTG_FS => usb::InterruptHandler<peripherals::USB_OTG_FS>;
});
static USB_EP: StaticCell<[u8; 256]> = StaticCell::new();
pub fn clock_config() -> hal::Config {
    common::f4_clocks()
}
/// Pin and DMA mapping are known, but source UART7 has no baud/format settings.
pub struct UnconfiguredUart7 {
    pub peripheral: hal::Peri<'static, peripherals::UART7>,
    pub rx: hal::Peri<'static, peripherals::PE7>,
    pub tx: hal::Peri<'static, peripherals::PE8>,
    pub rx_dma: hal::Peri<'static, peripherals::DMA1_CH3>,
}
impl UnconfiguredUart7 {
    pub fn into_receiver(
        self,
        config: usart::Config,
    ) -> Result<UartRx<'static>, usart::ConfigError> {
        usart::UartRx::new(self.peripheral, self.rx, self.rx_dma, Irqs, config).map(UartRx::new)
    }
}
pub struct Resources {
    pub can: [CanAdapter<'static>; 2],
    /// USART1 100k, USART6 9600, UART8 9600. Explicit 8N1 integration policy:
    /// the source does not record other format overrides for these ports.
    pub receive: [UartRx<'static>; 3],
    pub uart7: UnconfiguredUart7,
    pub motor_pwm: SimplePwm<'static, peripherals::TIM5>,
    pub auxiliary_pwm: SimplePwm<'static, peripherals::TIM8>,
    pub encoder: hal::timer::qei::Qei<'static, peripherals::TIM4>,
    pub usb: usb::Driver<'static, peripherals::USB_OTG_FS>,
    /// UART TX and actuator GPIOs.
    pub reserved: [Input<'static>; 18],
}
impl Resources {
    pub fn new(p: hal::Peripherals) -> Self {
        let can = common::bxcan_pair_loopback(
            can::Can::new(p.CAN1, p.PD0, p.PD1, Irqs),
            can::Can::new(p.CAN2, p.PB12, p.PB13, Irqs),
            10,
            3,
        );
        let receive = [
            UartRx::new(
                usart::UartRx::new(
                    p.USART1,
                    p.PB7,
                    p.DMA2_CH2,
                    Irqs,
                    common::uart_config(100_000),
                )
                .unwrap(),
            ),
            UartRx::new(
                usart::UartRx::new(p.USART6, p.PG9, p.DMA2_CH1, Irqs, common::uart_config(9600))
                    .unwrap(),
            ),
            UartRx::new(
                usart::UartRx::new(p.UART8, p.PE0, p.DMA1_CH6, Irqs, common::uart_config(9600))
                    .unwrap(),
            ),
        ];
        let mut motor_pwm = SimplePwm::new(
            p.TIM5,
            None,
            Some(PwmPin::new(p.PH11, OutputType::PushPull)),
            Some(PwmPin::new(p.PH12, OutputType::PushPull)),
            Some(PwmPin::new(p.PI0, OutputType::PushPull)),
            Hertz(50),
            Default::default(),
        );
        motor_pwm.ch2().disable();
        motor_pwm.ch3().disable();
        motor_pwm.ch4().disable();
        let mut auxiliary_pwm = SimplePwm::new(
            p.TIM8,
            None,
            None,
            Some(PwmPin::new(p.PI7, OutputType::PushPull)),
            Some(PwmPin::new(p.PI2, OutputType::PushPull)),
            Hertz(50),
            Default::default(),
        );
        auxiliary_pwm.ch3().disable();
        auxiliary_pwm.ch4().disable();
        let encoder = hal::timer::qei::Qei::new::<hal::timer::Ch1, hal::timer::Ch2>(
            p.TIM4,
            p.PD12,
            p.PD13,
            Default::default(),
        );
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
        let uart7 = UnconfiguredUart7 {
            peripheral: p.UART7,
            rx: p.PE7,
            tx: p.PE8,
            rx_dma: p.DMA1_CH3,
        };
        let pins: [hal::Peri<'static, hal::gpio::AnyPin>; 18] = [
            p.PA9.into(),
            p.PG14.into(),
            p.PE1.into(),
            p.PA1.into(),
            p.PA2.into(),
            p.PA3.into(),
            p.PA4.into(),
            p.PC0.into(),
            p.PC1.into(),
            p.PE15.into(),
            p.PF0.into(),
            p.PF1.into(),
            p.PF10.into(),
            p.PG13.into(),
            p.PH2.into(),
            p.PI5.into(),
            p.PI6.into(),
            p.PI9.into(),
        ];
        Self {
            can,
            receive,
            uart7,
            motor_pwm,
            auxiliary_pwm,
            encoder,
            usb,
            reserved: pins.map(common::reserve_input),
        }
    }
}
