// Construction probe board contract: a populated, accurate 24 MHz HSE crystal.
// SYSCLK/HCLK remain HSI64. No PLL is required by these peripheral selections.
fn probe_config() -> hal::Config {
    use hal::rcc::mux::{Fdcansel, Otgfssel, Persel, Spi1sel};
    use hal::rcc::{HSIPrescaler, Hse, HseMode, Sysclk};
    let mut config = hal::Config::default();
    config.rcc.hsi = Some(HSIPrescaler::Div1);
    config.rcc.sys = Sysclk::Hsi;
    config.rcc.hse = Some(Hse {
        freq: hal::time::Hertz(24_000_000),
        mode: HseMode::Oscillator,
    });
    config.rcc.mux.fdcansel = Fdcansel::Hse;
    config.rcc.mux.persel = Persel::Hsi;
    config.rcc.mux.spi1sel = Spi1sel::Per;
    config.rcc.hsi48 = Some(hal::rcc::Hsi48Config {
        sync_from_usb: true,
    });
    config.rcc.mux.otgfssel = Otgfssel::Hsi48;
    config
}
