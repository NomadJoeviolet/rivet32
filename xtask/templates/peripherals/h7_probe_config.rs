// Both images bind the same helper; only the primary configures shared clocks.
#[cfg_attr(embodied_core_cm4, allow(dead_code))]
fn probe_config() -> hal::Config {
    use hal::rcc::{Pll, PllDiv, PllMul, PllPreDiv, PllSource, mux};
    let mut config = hal::Config::default();
    // HSE24 / 15 = 1.6 MHz reference; x120 = 192 MHz VCO; /4 = 48 MHz Q.
    // This lies inside the 1..2 MHz / 150..420 MHz medium-VCO ranges in
    // H745 DS12923 and H747 DS12930 table 50 (also H755/H757 datasheets),
    // and selects HAL Range1 + MediumVco without relying on the 2 MHz edge.
    // System/bus clocks keep internal HSI defaults. Board supply must match LDO.
    // The probe board must supply an accurate 24 MHz HSE crystal.
    config.rcc.hse = Some(hal::rcc::Hse {
        freq: hal::time::Hertz(24_000_000),
        mode: hal::rcc::HseMode::Oscillator,
    });
    config.rcc.hsi48 = None;
    config.rcc.pll1 = Some(Pll {
        source: PllSource::Hse,
        prediv: PllPreDiv::Div15,
        mul: PllMul::Mul120,
        divp: None,
        divq: Some(PllDiv::Div4),
        divr: None,
    });
    config.rcc.mux.fdcansel = mux::Fdcansel::Pll1Q;
    config.rcc.mux.persel = mux::Persel::Hsi;
    config.rcc.mux.spi123sel = mux::Saisel::Per;
    config.rcc.mux.usbsel = mux::Usbsel::Pll1Q;
    config
}
