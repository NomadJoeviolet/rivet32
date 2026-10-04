//! STM32H723VGT6 reference configuration: PA9/PA10 are TIM1 outputs.
use super::common::{self, ImuBus, ImuSpi};
use embodied_framework::stm32::{can::CanAdapter, hal, uart::UartRx};
use hal::{
    adc, bind_interrupts, can, dma,
    gpio::{Input, Level, Output, OutputType, Speed},
    peripherals, spi,
    time::Hertz,
    timer::simple_pwm::{PwmPin, SimplePwm},
    usart, usb,
};
use static_cell::StaticCell;

pub const NAME: &str = "STM32H723VGT6";
bind_interrupts!(pub struct Irqs {
    USART3 => usart::InterruptHandler<peripherals::USART3>;
    UART7 => usart::InterruptHandler<peripherals::UART7>;
    UART8 => usart::InterruptHandler<peripherals::UART8>;
    USART10 => usart::InterruptHandler<peripherals::USART10>;
    DMA1_STREAM1 => dma::InterruptHandler<peripherals::DMA1_CH1>;
    DMA1_STREAM3 => dma::InterruptHandler<peripherals::DMA1_CH3>;
    DMA1_STREAM5 => dma::InterruptHandler<peripherals::DMA1_CH5>;
    DMA1_STREAM6 => dma::InterruptHandler<peripherals::DMA1_CH6>;
    FDCAN1_IT0 => can::IT0InterruptHandler<peripherals::FDCAN1>;
    FDCAN1_IT1 => can::IT1InterruptHandler<peripherals::FDCAN1>;
    FDCAN2_IT0 => can::IT0InterruptHandler<peripherals::FDCAN2>;
    FDCAN2_IT1 => can::IT1InterruptHandler<peripherals::FDCAN2>;
    FDCAN3_IT0 => can::IT0InterruptHandler<peripherals::FDCAN3>;
    FDCAN3_IT1 => can::IT1InterruptHandler<peripherals::FDCAN3>;
    OTG_HS => usb::InterruptHandler<peripherals::USB_OTG_HS>;
});
static USB_EP: StaticCell<[u8; 256]> = StaticCell::new();

/// Exact current Core clock tree. At 540MHz the chip's CPU_FREQ_BOOST option
/// must be set; pinned HAL checks this, and this code never programs option bytes.
pub fn clock_config() -> hal::Config {
    use hal::rcc::*;
    let mut c = hal::Config::default();
    c.rcc.hse = Some(Hse {
        freq: Hertz(12_000_000),
        mode: HseMode::Oscillator,
    });
    c.rcc.pll1 = Some(Pll {
        source: PllSource::Hse,
        prediv: PllPreDiv::Div1,
        mul: PllMul::Mul45,
        divp: Some(PllDiv::Div1),
        divq: Some(PllDiv::Div3),
        divr: Some(PllDiv::Div3),
    });
    c.rcc.pll2 = Some(Pll {
        source: PllSource::Hse,
        prediv: PllPreDiv::Div1,
        mul: PllMul::Mul20,
        divp: Some(PllDiv::Div3),
        divq: Some(PllDiv::Div3),
        divr: Some(PllDiv::Div2),
    });
    c.rcc.pll3 = Some(Pll {
        source: PllSource::Hse,
        prediv: PllPreDiv::Div1,
        mul: PllMul::Mul27,
        divp: Some(PllDiv::Div3),
        divq: Some(PllDiv::Div2),
        divr: Some(PllDiv::Div2),
    });
    c.rcc.sys = Sysclk::Pll1P;
    c.rcc.ahb_pre = AHBPrescaler::Div2;
    c.rcc.apb1_pre = APBPrescaler::Div2;
    c.rcc.apb2_pre = APBPrescaler::Div2;
    c.rcc.apb3_pre = APBPrescaler::Div2;
    c.rcc.apb4_pre = APBPrescaler::Div2;
    c.rcc.voltage_scale = VoltageScale::Scale0;
    c.rcc.supply_config = SupplyConfig::LDO;
    c.rcc.hsi48 = Some(Hsi48Config {
        sync_from_usb: false,
    });
    c.rcc.mux.spi123sel = mux::Saisel::Pll3P;
    c.rcc.mux.fdcansel = mux::Fdcansel::Pll2Q;
    c.rcc.mux.usbsel = mux::Usbsel::Hsi48;
    c.rcc.mux.adcsel = mux::Adcsel::Pll2P;
    c
}

pub struct Resources {
    pub can: [CanAdapter<'static>; 3],
    /// USART3, UART7, UART8, USART10, in this order.
    pub receive: [UartRx<'static>; 4],
    pub imu: ImuBus,
    pub display_spi: ImuSpi,
    pub servo_pwm: SimplePwm<'static, peripherals::TIM1>,
    pub auxiliary_pwm: SimplePwm<'static, peripherals::TIM4>,
    pub usb: usb::Driver<'static, peripherals::USB_OTG_HS>,
    pub adc: adc::Adc<'static, peripherals::ADC1, hal::mode::Blocking>,
    pub adc_pin: hal::Peri<'static, peripherals::PC4>,
    /// Original ADC circular DMA resource, reserved for an explicit stream setup.
    pub adc_dma: hal::Peri<'static, peripherals::DMA1_CH0>,
    /// UART TX, camera trigger, LCD DC and MCO: preserved without driving.
    pub reserved: [Input<'static>; 7],
}
impl Resources {
    pub fn new(p: hal::Peripherals) -> Self {
        // This board uses the _C package pads digitally. Keep their analog
        // switches closed, as in the source's unsplit PC2/PC3 GPIO configuration.
        hal::pac::SYSCFG.pmcr().modify(|w| {
            w.set_pc2so(false);
            w.set_pc3so(false);
        });
        let nominal = (1, 59, 20, 4);
        let data = (1, 12, 3, 3);
        let can = [
            common::fdcan_loopback(
                can::CanConfigurator::new(p.FDCAN1, p.PD0, p.PD1, Irqs),
                nominal,
                data,
            ),
            common::fdcan_loopback(
                can::CanConfigurator::new(p.FDCAN2, p.PB5, p.PB6, Irqs),
                nominal,
                data,
            ),
            common::fdcan_loopback(
                can::CanConfigurator::new(p.FDCAN3, p.PD12, p.PD13, Irqs),
                nominal,
                data,
            ),
        ];
        let receive = [
            UartRx::new(
                usart::UartRx::new(
                    p.USART3,
                    p.PD9,
                    p.DMA1_CH3,
                    Irqs,
                    common::uart_config(921_600),
                )
                .unwrap(),
            ),
            UartRx::new(
                usart::UartRx::new(
                    p.UART7,
                    p.PE7,
                    p.DMA1_CH1,
                    Irqs,
                    common::uart_config(921_600),
                )
                .unwrap(),
            ),
            UartRx::new(
                usart::UartRx::new(
                    p.UART8,
                    p.PE0,
                    p.DMA1_CH5,
                    Irqs,
                    common::uart_config(115_200),
                )
                .unwrap(),
            ),
            UartRx::new(
                usart::UartRx::new(
                    p.USART10,
                    p.PE2,
                    p.DMA1_CH6,
                    Irqs,
                    common::uart_config(115_200),
                )
                .unwrap(),
            ),
        ];
        let mut spi_config = spi::Config::default();
        // Source used 108MHz/8=13.5MHz, above BMI088's 10MHz limit. Keep
        // source evidence, but use the next divider for this actual IMU port.
        spi_config.frequency = Hertz(6_750_000);
        let imu = ImuBus {
            spi: spi::Spi::new_blocking(p.SPI2, p.PB13, p.PC1, p.PC2, spi_config),
            accelerometer_cs: Output::new(p.PC0, Level::High, Speed::Low),
            gyroscope_cs: Output::new(p.PC3, Level::High, Speed::Low),
        };
        spi_config.frequency = Hertz(6_750_000); // Core SPI1: 108MHz / 16, TX only.
        let display_spi = spi::Spi::new_blocking_txonly(p.SPI1, p.PB3, p.PD7, spi_config);
        let mut servo_pwm = SimplePwm::new(
            p.TIM1,
            None,
            Some(PwmPin::new(p.PA9, OutputType::PushPull)),
            Some(PwmPin::new(p.PA10, OutputType::PushPull)),
            None,
            Hertz(50),
            Default::default(),
        );
        servo_pwm.ch2().disable();
        servo_pwm.ch3().disable();
        let mut auxiliary_pwm = SimplePwm::new(
            p.TIM4,
            None,
            Some(PwmPin::new(p.PB7, OutputType::PushPull)),
            None,
            None,
            Hertz(15),
            Default::default(),
        );
        auxiliary_pwm.ch2().disable();
        auxiliary_pwm.set_period_us(65_536);
        let mut config = usb::Config::default();
        config.vbus_detection = false;
        let usb = usb::Driver::new_fs(
            p.USB_OTG_HS,
            p.PA12,
            p.PA11,
            Irqs,
            USB_EP.init([0; 256]),
            config,
        );
        let mut adc_config = adc::Config::default();
        // Current Core uses ADC1 IN4, 16 bit, PLL2P / 64 = 1.25MHz.
        adc_config.resolution = Some(adc::Resolution::Bits16);
        adc_config.clock = adc::Clock::Async(adc::Prescaler::Div64);
        let adc = adc::Adc::new_blocking(p.ADC1, adc_config);
        let pins: [hal::Peri<'static, hal::gpio::AnyPin>; 7] = [
            p.PD8.into(),
            p.PE8.into(),
            p.PE1.into(),
            p.PE3.into(),
            p.PA5.into(),
            p.PD10.into(),
            p.PA8.into(),
        ];
        let reserved = pins.map(common::reserve_input);
        Self {
            can,
            receive,
            imu,
            display_spi,
            servo_pwm,
            auxiliary_pwm,
            usb,
            adc,
            adc_pin: p.PC4,
            adc_dma: p.DMA1_CH0,
            reserved,
        }
    }

    /// Explicit one-shot conversion. The idle reference image does not start
    /// the source firmware's autonomous circular DMA acquisition.
    pub fn read_adc(&mut self) -> u16 {
        self.adc
            .blocking_read(&mut self.adc_pin, adc::SampleTime::Cycles325)
    }
}
