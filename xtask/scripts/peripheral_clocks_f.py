"""F-series constructor clocks using Embassy's existing RCC configuration API."""
import re


def oscillator(peripheral, package):
    if package is None:
        raise ValueError('HSE requires an exact package')
    selected = []
    for signal in ['OSC_IN', 'OSC_OUT']:
        candidates = set()
        for pad in package['pins']:
            # Some F1 packages have dedicated oscillator pads, not GPIOs.
            if signal in pad['signals']:
                candidates.add((signal, pad['position']))
            for pin in peripheral.get('pins', []):
                if pin['signal'] == signal and pin['pin'] in pad['signals']:
                    candidates.add((pin['pin'], pad['position']))
        if len(candidates) != 1:
            raise ValueError('HSE requires one bonded ' + signal + ' in ' + package['name'])
        pin, position = candidates.pop()
        selected.append(dict(signal=signal, pin=pin, position=position))
    if selected[0]['position'] == selected[1]['position']:
        raise ValueError('HSE oscillator pins must use distinct package positions')
    return selected


def configuration(data, feature, paired, package):
    families = {'STM32F0', 'STM32F1', 'STM32F2', 'STM32F3', 'STM32F4', 'STM32F7'}
    if paired or data is None or data.get('family') not in families:
        return None
    if data.get('name', '').lower() != feature or len(data['cores']) != 1:
        raise ValueError('clock recipe requires exact single-core chip identity')
    family = data['family']
    peris = {p['name']: p for p in data['cores'][0]['peripherals']}
    rcc = peris['RCC']
    version = rcc['registers']['version']
    allowed = {'STM32F0': {'f0v1', 'f0v2', 'f0v3', 'f0v4'},
               'STM32F1': {'f1', 'f100', 'f1cl'}, 'STM32F2': {'f2'},
               'STM32F3': {'f37', 'f3v1', 'f3v2', 'f3v3'},
               'STM32F4': {'f4', 'f410'}, 'STM32F7': {'f7'}}
    if version not in allowed[family]:
        raise ValueError('RCC version does not match F chip family identity')
    usb = any(p.get('registers', {}).get('kind') in {'usb', 'otg'} for p in peris.values())
    crs = 'CRS' in peris
    f0 = family == 'STM32F0'
    small_pll = family in {'STM32F0', 'STM32F1', 'STM32F3'}
    hsi = 8_000_000 if small_pll else 16_000_000
    system = 48_000_000 if usb else hsi
    apb1_div = 2 if usb and not f0 else 1
    details = dict(profile='f-internal-hsi', status='explicit-kernel-clock-recipe', rcc_version=version,
                   hsi_hz=hsi, sys_hz=system, hclk_hz=system, pclk1_hz=system // apb1_div,
                   pclk2_hz=system, pclk1_tim_hz=system, pclk2_tim_hz=system,
                   mux={}, reserved_pins=[], usb_crs_enabled=False,
                   scope='Cold boot constructor probes; board clock accuracy, USB transfers and HIL unverified')
    statements = ['config.rcc.hsi = true;',
                  'config.rcc.ahb_pre = hal::rcc::AHBPrescaler::Div1;',
                  f'config.rcc.apb1_pre = hal::rcc::APBPrescaler::Div{apb1_div};']
    if not f0:
        statements.append('config.rcc.apb2_pre = hal::rcc::APBPrescaler::Div1;')
    if not usb:
        statements.append('config.rcc.sys = hal::rcc::Sysclk::Hsi;')
        if crs:
            statements.append('config.rcc.hsi48 = None;')
    elif crs:
        if version != 'f0v4':
            raise ValueError('No reviewed F crystal-free USB profile for this RCC')
        details.update(profile='f0-hsi48-crs-usb', usb_hz=48_000_000, usb_crs_enabled=True)
        details['mux']['usbsw'] = 'Hsi48'
        statements += ['config.rcc.hsi48 = Some(hal::rcc::Hsi48Config { sync_from_usb: true });',
                       'config.rcc.sys = hal::rcc::Sysclk::Hsi48;',
                       'config.rcc.mux.usbsw = hal::rcc::mux::Usbsw::Hsi48;']
    else:
        pins = oscillator(rcc, package)
        details.update(profile='f-hse8-usb48', usb_hz=48_000_000,
                       reserved_pins=[p['pin'] for p in pins if re.fullmatch(r'P[A-Z]\d+', p['pin'])],
                       board_requirements=dict(hse_hz=8_000_000, hse_mode='crystal', hse_pins=pins,
                                               usb_supply='3.0 to 3.6 V; PLL output must meet USB frequency tolerance'))
        statements += ['''config.rcc.hse = Some(hal::rcc::Hse {
        freq: hal::time::Hertz(8_000_000),
        mode: hal::rcc::HseMode::Oscillator,
    });''', 'config.rcc.sys = hal::rcc::Sysclk::Pll1P;']
        if small_pll:
            statements.append('''config.rcc.pll = Some(hal::rcc::Pll {
        src: hal::rcc::PllSource::HSE,
        prediv: hal::rcc::PllPreDiv::Div1,
        mul: hal::rcc::PllMul::Mul6,
    });''')
            details['pll1'] = dict(source='HSE8', predivider=1, multiplier=6, p_hz=48_000_000)
            if f0:
                statements.append('config.rcc.mux.usbsw = hal::rcc::mux::Usbsw::Pll1P;')
                details['mux']['usbsw'] = 'Pll1P'
            else:
                # f013.rs sets USBPRE from the 48 MHz PLL output. On F1CL,
                # the PAC Div1 name is the VCO/2 setting (VCO = 2 * PLLCLK).
                details['usb_prescaler'] = 'HAL f013 selects USBPRE::Div1 for PLLCLK48'
        else:
            statements += ['config.rcc.pll_src = hal::rcc::PllSource::Hse;',
                           '''config.rcc.pll = Some(hal::rcc::Pll {
        prediv: hal::rcc::PllPreDiv::Div8,
        mul: hal::rcc::PllMul::Mul192,
        divp: Some(hal::rcc::PllPDiv::Div4),
        divq: Some(hal::rcc::PllQDiv::Div4),
        divr: None,
    });''']
            details['pll1'] = dict(source='HSE8', predivider=8, multiplier=192, p_divider=4,
                                   q_divider=4, reference_hz=1_000_000, vco_hz=192_000_000,
                                   p_hz=48_000_000, q_hz=48_000_000)
            if family in {'STM32F4', 'STM32F7'}:
                statements.append('config.rcc.mux.clk48sel = hal::rcc::mux::Clk48sel::Pll1Q;')
                details['mux']['clk48sel'] = 'Pll1Q'
    code = ('// Constructor-probe clocks; see the accompanying board_requirements.\n'
            'fn probe_config() -> hal::Config {\n    let mut config = hal::Config::default();\n    '
            + '\n    '.join(statements) + '\n    config\n}\n')
    return code, details
