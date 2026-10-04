"""Clock recipes for exact H5/H7 metadata, preserving separate patched/dual recipes.

These are constructor-probe board configurations. They do not establish physical
clock accuracy, successful startup, USB enumeration, or CAN communication.
"""

H5_VERSIONS = {'h50', 'h5'}
H7_VERSIONS = {'h7rm0433', 'h7rm0468', 'h7ab', 'h7rs'}


def configuration(data, feature, paired, package):
    if paired or data is None:
        return None
    family = data.get('family')
    if family not in {'STM32H5', 'STM32H7'}:
        return None
    if data.get('name', '').lower() != feature or len(data['cores']) != 1:
        raise ValueError('clock recipe requires exact single-core chip identity')
    core = data['cores'][0]
    rcc = next(p for p in core['peripherals'] if p['name'] == 'RCC')
    version = rcc['registers']['version']
    if version not in H5_VERSIONS | H7_VERSIONS:
        return None
    if (version in H5_VERSIONS) != (family == 'STM32H5'):
        raise ValueError('RCC version does not match chip family identity')

    statements = [
        'config.rcc.hsi = Some(hal::rcc::HSIPrescaler::Div1);',
        'config.rcc.sys = hal::rcc::Sysclk::Hsi;',
    ]
    details = dict(status='explicit-kernel-clock-recipe', rcc_version=version,
                   sys_hz=64_000_000, hclk_hz=64_000_000, spi1_hz=64_000_000,
                   usb_hz=48_000_000, usb_crs_enabled=True, reserved_pins=[],
                   scope='Cold boot constructor probes; oscillator accuracy/startup, USB SOF calibration and transfers require HIL')
    if version in H5_VERSIONS:
        # DS14053 table 44, DS14540 table 47, DS14539 table 48,
        # DS14258 table 49 and DS14121 table 50: 1..2 MHz input,
        # 150..420 MHz medium VCO. Integer mode uses 1.6 -> 192 -> 48 MHz.
        statements += [
            '''config.rcc.pll1 = Some(hal::rcc::Pll {
        source: hal::rcc::PllSource::Hsi,
        prediv: hal::rcc::PllPreDiv::Div40,
        mul: hal::rcc::PllMul::Mul120,
        divp: None,
        divq: Some(hal::rcc::PllDiv::Div4),
        divr: None,
    });''',
            'config.rcc.mux.fdcan12sel = hal::rcc::mux::Fdcansel::Pll1Q;',
            'config.rcc.mux.persel = hal::rcc::mux::Persel::Hsi;',
            'config.rcc.mux.spi1sel = hal::rcc::mux::' + ('Spisel' if version == 'h50' else 'Spi1sel') + '::Per;',
            'config.rcc.mux.usbsel = hal::rcc::mux::Usbsel::Hsi48;',
        ]
        details.update(profile='h5-internal-pll48-hsi64', fdcan_hz=48_000_000,
                       pll1=dict(source='HSI64', predivider=40, multiplier=120, q_divider=4,
                                 reference_hz=1_600_000, vco_hz=192_000_000, q_hz=48_000_000,
                                 range='Range1/MediumVco', fractional=False),
                       board_requirements=dict(trustzone='disabled'),
                       mux=dict(fdcan12sel='Pll1Q', persel='Hsi', spi1sel='Per', usbsel='Hsi48'))
    else:
        if package is None:
            raise ValueError('HSE recipe requires an exact package')
        bonded = {signal for p in package['pins'] for signal in p['signals']}
        bonded &= {p['name'] for p in core['pins']}
        oscillator = {}
        for signal in ('OSC_IN', 'OSC_OUT'):
            pins = {p['pin'] for p in rcc.get('pins', []) if p['signal'] == signal and p['pin'] in bonded}
            if len(pins) != 1:
                raise ValueError('HSE crystal requires one bonded ' + signal + ' pin in ' + package['name'])
            oscillator[signal] = pins.pop()
        if len(set(oscillator.values())) != 2:
            raise ValueError('HSE crystal pins must be distinct')
        statements += [
            '''config.rcc.hse = Some(hal::rcc::Hse {
        freq: hal::time::Hertz(24_000_000),
        mode: hal::rcc::HseMode::Oscillator,
    });''',
            'config.rcc.mux.fdcansel = hal::rcc::mux::Fdcansel::Hse;',
            'config.rcc.mux.persel = hal::rcc::mux::Persel::Hsi;',
        ]
        # The generic HSI48 USB SOF setting selects USB1 on classic H7,
        # whereas these probes can use USB2 FS. Use the already-required
        # accurate HSE crystal through PLL3Q instead of guessing a CRS route.
        extra = ''
        if feature.startswith(('stm32h743', 'stm32h730')):
            extra += '        fracn: None,\n'
        if version == 'h7rs':
            extra += '        divs: None,\n        divt: None,\n'
        statements.append('''config.rcc.pll3 = Some(hal::rcc::Pll {
        source: hal::rcc::PllSource::Hse,
        prediv: hal::rcc::PllPreDiv::Div15,
        mul: hal::rcc::PllMul::Mul120,
        divp: None,
        divq: Some(hal::rcc::PllDiv::Div4),
        divr: None,
''' + extra + '    });')
        details.update(usb_crs_enabled=False,
                       pll3=dict(source='HSE24', predivider=15, multiplier=120, q_divider=4,
                                 reference_hz=1_600_000, vco_hz=192_000_000, q_hz=48_000_000,
                                 range='Range1/MediumVco', fractional=False))
        if version == 'h7rs':
            statements += [
                'config.rcc.mux.spi1sel = hal::rcc::mux::Spi123sel::Per;',
                'config.rcc.mux.usb_otg_fssel = hal::rcc::mux::UsbOtgFssel::Pll3Q;',
            ]
            mux = dict(fdcansel='Hse', persel='Hsi', spi1sel='Per',
                       usb_otg_fssel='Pll3Q')
            # Embassy exposes the PHY clock only when the exact chip has USB HS.
            if any(p['name'] == 'USB_OTG_HS' for p in core['peripherals']):
                statements.append('config.rcc.mux.usbphycsel = hal::rcc::Usbphycsel::Hse;')
                mux['usbphycsel'] = 'Hse'
                details['usb_phy_reference_hz'] = 24_000_000
        else:
            statements += [
                'config.rcc.mux.spi123sel = hal::rcc::mux::Saisel::Per;',
                'config.rcc.mux.usbsel = hal::rcc::mux::Usbsel::Pll3Q;',
            ]
            mux = dict(fdcansel='Hse', persel='Hsi', spi123sel='Per', usbsel='Pll3Q')
        details.update(profile='h7-hse24-pll3q48-hsi64', fdcan_hz=24_000_000, mux=mux,
                       reserved_pins=sorted(oscillator.values()),
                       board_requirements=dict(hse_hz=24_000_000, hse_mode='crystal',
                                               hse_pins=oscillator, core_supply='LDO'))
    statements.append('config.rcc.hsi48 = Some(hal::rcc::Hsi48Config { sync_from_usb: true });'
                      if version in H5_VERSIONS else 'config.rcc.hsi48 = None;')
    code = ('// Constructor-probe clocks; see the accompanying board_requirements.\n'
            'fn probe_config() -> hal::Config {\n'
            '    let mut config = hal::Config::default();\n    '
            + '\n    '.join(statements) + '\n    config\n}\n')
    return code, details
