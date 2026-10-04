"""G0/G4 constructor clocks selected from exact peripheral and package metadata."""


def configuration(data, feature, paired, package):
    if paired or data is None or data.get('family') not in {'STM32G0','STM32G4'}:
        return None
    if data.get('name','').lower()!=feature or len(data['cores'])!=1:
        raise ValueError('clock recipe requires exact single-core chip identity')
    core=data['cores'][0]
    peripherals={p['name']:p for p in core['peripherals']}
    rcc=peripherals['RCC']; version=rcc['registers']['version']
    if version not in {'g0x0','g0x1','g4'}:
        return None
    g4=version=='g4'
    if g4 != (data['family']=='STM32G4'):
        raise ValueError('RCC version does not match chip family identity')
    usb='USB' in peripherals
    crs='CRS' in peripherals
    fdcan=any(p.startswith('FDCAN') for p in peripherals)
    statements=[
        'config.rcc.hsi = true;' if g4 else
        'config.rcc.hsi = Some(hal::rcc::Hsi { sys_div: hal::rcc::HsiSysDiv::Div1 });',
        'config.rcc.sys = hal::rcc::Sysclk::Hsi;',
        'config.rcc.ahb_pre = hal::rcc::AHBPrescaler::Div1;',
        'config.rcc.apb1_pre = hal::rcc::APBPrescaler::Div1;',
    ]
    if g4:
        statements.append('config.rcc.apb2_pre = hal::rcc::APBPrescaler::Div1;')
    details=dict(profile='g-internal-hsi16',status='explicit-kernel-clock-recipe',rcc_version=version,
                 sys_hz=16_000_000,hclk_hz=16_000_000,spi1_hz=16_000_000,
                 mux={},reserved_pins=[],usb_crs_enabled=False,
                 scope='Cold boot constructor probes; HSI accuracy, clock startup and transfers require HIL')
    if fdcan:
        statements.append('config.rcc.mux.fdcansel = hal::rcc::mux::Fdcansel::Pclk1;')
        details['mux']['fdcansel']='Pclk1'
        details['fdcan_hz']=16_000_000
    if usb:
        details['usb_hz']=48_000_000
        if crs:
            statements.append('config.rcc.hsi48 = Some(hal::rcc::Hsi48Config { sync_from_usb: true });')
            field,enum=('clk48sel','Clk48sel') if g4 else ('usbsel','Usbsel')
            statements.append('config.rcc.mux.'+field+' = hal::rcc::mux::'+enum+'::Hsi48;')
            details['mux'][field]='Hsi48'
            details['usb_crs_enabled']=True
        else:
            if version!='g0x0':
                raise ValueError('USB without CRS has no reviewed clock recipe for this RCC')
            if package is None:
                raise ValueError('HSE bypass recipe requires an exact package')
            bonded={s for p in package['pins'] for s in p['signals']}
            bonded &= {p['name'] for p in core['pins']}
            pins={p['pin'] for p in rcc.get('pins',[]) if p['signal']=='OSC_IN' and p['pin'] in bonded}
            if len(pins)!=1:
                raise ValueError('HSE bypass requires one bonded OSC_IN pin in '+package['name'])
            pin=pins.pop()
            # DS13565: external HSE for USB; 8 MHz input, 192 MHz VCO.
            # ST correction 217013: USBSEL PLLQ is 2, HSE is 1, matching PAC.
            statements += [
                '''config.rcc.hse = Some(hal::rcc::Hse {
        freq: hal::time::Hertz(8_000_000),
        mode: hal::rcc::HseMode::Bypass,
    });''',
                '''config.rcc.pll = Some(hal::rcc::Pll {
        source: hal::rcc::PllSource::Hse,
        prediv: hal::rcc::PllPreDiv::Div1,
        mul: hal::rcc::PllMul::Mul24,
        divp: None,
        divq: Some(hal::rcc::PllQDiv::Div4),
        divr: None,
    });''',
                'config.rcc.mux.usbsel = hal::rcc::mux::Usbsel::Pll1Q;',
                'config.rcc.ls = hal::rcc::LsConfig::default_lsi();',
            ]
            details.update(profile='g0b0-hse8-bypass-usb-pll48',reserved_pins=[pin],
                           board_requirements=dict(hse_hz=8_000_000,hse_mode='bypass',hse_pin=pin,
                                                   hse_duty_cycle_percent=[45,55],
                                                   low_speed_source='LSI; no retained LSE oscillator on the shared OSC_IN pin'),
                           pll1=dict(source='HSE8',predivider=1,multiplier=24,q_divider=4,
                                     reference_hz=8_000_000,vco_hz=192_000_000,q_hz=48_000_000))
            details['mux']['usbsel']='Pll1Q'
    code=('// Constructor-probe clocks; see the accompanying board_requirements.\n'
          'fn probe_config() -> hal::Config {\n    let mut config = hal::Config::default();\n    '
          +'\n    '.join(statements)+'\n    config\n}\n')
    return code,details
