"""Exact-package, constructor-bearing recipes for the pinned Embassy HAL.

This module never infers hardware from the family name. A successful plan is
only permission to attempt a build; it is not a passing peripheral test.
"""
import itertools
from pathlib import Path
import re

from peripheral_clocks_f import configuration as f_clock_configuration

from prepare_h5_47a_pac import EXACT_CHIPS as H5_47A_FEATURES
from peripheral_clocks_h import configuration as h_clock_configuration
from peripheral_clocks_g import configuration as g_clock_configuration

CATEGORIES = ("gpio", "uart", "can", "usb_cdc", "spi", "pwm", "timer")
GENERAL_TIMERS = {"TIM_1CH", "TIM_2CH", "TIM_GP16", "TIM_GP32", "TIM_1CH_CMP", "TIM_2CH_CMP", "TIM_ADV"}
TIME_TIMERS = {1, 2, 3, 4, 5, 8, 9, 12, 15, 19, 20, 21, 22, 23, 24}
DEFAULT_CLOCK_SOURCE = "fn probe_config() -> hal::Config { hal::Config::default() }\n"
H5_47C_FEATURES = frozenset(('stm32h543ce', 'stm32h543cg', 'stm32h543re', 'stm32h543rg',
                           'stm32h543ug', 'stm32h543ve', 'stm32h543vg', 'stm32h543ze', 'stm32h543zg',
                           'stm32h553cg', 'stm32h553rg', 'stm32h553ug', 'stm32h553vg', 'stm32h553zg'))


def legacy_crystal_pins(data, feature, package):
    # Bare clock_configuration calls expose the template only. plan() always
    # supplies exact metadata/package and must bind physical crystal pins.
    if data is None or package is None:
        return {}
    if data.get('name', '').lower() != feature.split('-')[0]:
        raise ValueError('clock recipe requires exact chip identity')
    core = next((c for c in data['cores'] if c['name'] == feature.rsplit('-', 1)[-1]), data['cores'][0])
    rcc = next(p for p in core['peripherals'] if p['name'] == 'RCC')
    bonded = {signal for pin in package['pins'] for signal in pin['signals']}
    bonded &= {pin['name'] for pin in core['pins']}
    oscillator = {}
    for signal in ['OSC_IN', 'OSC_OUT']:
        options = {p['pin'] for p in rcc.get('pins', []) if p['signal'] == signal and p['pin'] in bonded}
        if len(options) != 1:
            raise ValueError('HSE crystal requires one bonded ' + signal + ' in ' + package['name'])
        oscillator[signal] = options.pop()
    if len(set(oscillator.values())) != 2:
        raise ValueError('HSE crystal pins must be distinct')
    return oscillator


def clock_configuration(feature, paired, data=None, package=None):
    candidate = f_clock_configuration(data, feature, paired, package)
    if candidate is not None:
        return candidate
    additional = g_clock_configuration(data, feature, paired, package)
    if additional is not None:
        return additional
    additional = h_clock_configuration(data, feature, paired, package)
    if additional is not None:
        return additional
    if not paired and feature in H5_47C_FEATURES:
        code = (Path(__file__).resolve().parents[1] / 'templates/peripherals/h5_47c_probe_config.rs').read_text(encoding='utf-8')
        return code, dict(profile='h5-47c-internal-oscillators', status='explicit-kernel-clock-recipe',
                         pll1=dict(source='HSI64', predivider=40, multiplier=120, q_divider=4,
                                   reference_hz=1_600_000, vco_hz=192_000_000, range='Range1/MediumVco'),
                         sys_hz=64_000_000, fdcan_hz=48_000_000, spi1_hz=64_000_000, usb_hz=48_000_000,
                         mux=dict(fdcan12sel='Pll1Q', persel='Hsi', spi1sel='Per', usbsel='Hsi48'),
                         usb_crs_enabled=True,
                         scope='Cold boot, TrustZone disabled; retained RTC source switching and HIL not verified')
    if not paired and feature in H5_47A_FEATURES:
        oscillator = legacy_crystal_pins(data, feature, package)
        code = (Path(__file__).resolve().parents[1] / 'templates/peripherals/h5_47a_probe_config.rs').read_text(encoding='utf-8')
        return code, dict(profile='h5-47a-hse24-hsi64', status='explicit-kernel-clock-recipe',
                         board_requirements=dict(hse_hz=24_000_000, hse_mode='crystal', hse_pins=oscillator),
                         reserved_pins=sorted(oscillator.values()),
                         sys_hz=64_000_000, fdcan_hz=24_000_000, spi1_hz=64_000_000, usb_hz=48_000_000,
                         mux=dict(fdcansel='Hse', persel='Hsi', spi1sel='Per', otgfssel='Hsi48'),
                         usb_crs_enabled=True, usb_hs_phy_enabled=False,
                         scope='Cold boot, TrustZone disabled, USB FS; HSE startup and USB SOF calibration require HIL')
    if not paired:
        return DEFAULT_CLOCK_SOURCE, dict(profile="hal-default", status="kernel-clock-selection-not-yet-reviewed")
    if not re.fullmatch(r"stm32h7(?:45|47|55|57)[a-z][gi]-cm[47]", feature):
        raise ValueError("no verified paired clock recipe for " + feature)
    oscillator = legacy_crystal_pins(data, feature, package)
    code = (Path(__file__).resolve().parents[1] / "templates/peripherals/h7_probe_config.rs").read_text(encoding="utf-8")
    return code, dict(profile="h7-dual-hse24-pll1q48", status="explicit-kernel-clock-recipe",
                     board_requirements=dict(hse_hz=24_000_000, hse_mode="crystal", hse_pins=oscillator, core_supply="LDO"),
                     reserved_pins=sorted(oscillator.values()), usb_crs_enabled=False,
                     pll1=dict(source="HSE24", predivider=15, multiplier=120, q_divider=4,
                               reference_hz=1_600_000, vco_hz=192_000_000, q_hz=48_000_000, range="Range1/MediumVco"),
                     sys_hz=64_000_000, fdcan_hz=48_000_000, spi123_hz=64_000_000, usb_hz=48_000_000,
                     mux=dict(fdcansel="Pll1Q", persel="Hsi", spi123sel="Per", usbsel="Pll1Q"),
                     datasheets=["https://www.st.com/resource/en/datasheet/" + part + ".pdf"
                                 for part in ("stm32h745xi", "stm32h747xg", "stm32h755ii", "stm32h757bi")],
                     scope="PLL/routing configuration reviewed; oscillator startup and board behavior not exercised")


def fdcan(registers):
    return registers.get("kind") == "can" and registers.get("version") in {"fdcan_v1", "fdcan_v2"}


def pin_choices(peripheral, signals, bonded):
    choices = [[p for p in peripheral.get("pins", []) if p["signal"] == signal and p["pin"] in bonded]
               for signal in signals]
    for pins in itertools.product(*choices):
        names = {p["pin"] for p in pins}
        if len(names) != len(pins):
            continue
        mappings = peripheral.get("afio", {}).get("values", [None])
        for mapping in mappings:
            if mapping is None or names.issubset(mapping["pins"]):
                yield {"pins": dict(zip(signals, pins)), "remap": None if mapping is None else mapping["value"]}


def irq(peripheral, signal):
    values = {v["interrupt"].upper() for v in peripheral.get("interrupts", []) if v["signal"] == signal}
    return sorted(values)[0] if values else None


def dma_pair(core, peripheral):
    controllers = {p["name"]: p for p in core.get("peripherals", [])}
    def candidates(signal):
        for route in peripheral.get("dma_channels", []):
            if route["signal"] != signal:
                continue
            for channel in core.get("dma_channels", []):
                if not ((route.get("channel") == channel["name"])
                        or (route.get("dmamux") and route["dmamux"] == channel.get("dmamux"))
                        or (route.get("dma") and route["dma"] == channel.get("dma"))):
                    continue
                interrupt = irq(controllers.get(channel.get("dma"), {}), channel["name"].split("_")[-1])
                if interrupt:
                    yield dict(channel, interrupt=interrupt, route=route)
    for tx, rx in itertools.product(candidates("TX"), candidates("RX")):
        if tx["name"] != rx["name"]:
            return tx, rx
    return None


def afio_type(peripheral, choice, widths):
    if "afio" not in peripheral:
        return "hal::gpio::AfioRemapNotApplicable"
    afio = peripheral["afio"]
    width = widths.get((afio["register"], afio["field"]))
    if width is None:
        raise ValueError("AFIO remap field width has no verified source")
    value = choice["remap"]
    return (f"hal::gpio::AfioRemapBool<{'true' if value else 'false'}>" if width == 1
            else f"hal::gpio::AfioRemap<{value}>")


def plan(data, feature, package=None, afio_widths=None, *, paired=False):
    if len(data["cores"]) != 1 and not paired:
        return {c: {"status": "blocked", "reason": "dual-core probe requires a paired resource-ownership recipe", "hardware_present": None}
                for c in CATEGORIES}
    if paired:
        if (not re.fullmatch(r"stm32h7(?:45|47|55|57)[a-z][gi]-cm[47]", feature)
                or data["name"].lower() != feature.rsplit("-", 1)[0]
                or {c["name"] for c in data["cores"]} != {"cm7", "cm4"}):
            raise ValueError("paired recipe requires the exact H7 part and both real cores")
        core = next(c for c in data["cores"] if c["name"] == feature.rsplit("-", 1)[1])
    else:
        core = data["cores"][0]
    packages = sorted(data["packages"], key=lambda p: p["name"])
    selected = next((p for p in packages if p["name"] == package), None) if package else packages[0]
    if selected is None:
        raise ValueError(f"exact package is absent: {package}")
    clock_source, clock_plan = clock_configuration(feature, paired, data=data, package=selected)
    bonded = {s for p in selected["pins"] for s in p["signals"] if re.fullmatch(r"P[A-Z]\d+", s)}
    bonded &= {p["name"] for p in core.get("pins", [])}
    bonded -= set(clock_plan.get('reserved_pins', []))
    peripherals = sorted(core["peripherals"], key=lambda p: p["name"])
    timer_candidates = [p for p in peripherals if p.get("registers", {}).get("block") in GENERAL_TIMERS
                        and p["name"].startswith("TIM") and int(p["name"][3:]) in TIME_TIMERS and irq(p, "CC")]
    rank = {"TIM_GP32": 0, "TIM_2CH": 1, "TIM_2CH_CMP": 2, "TIM_GP16": 3, "TIM_ADV": 4}
    timer_candidates.sort(key=lambda p: (rank.get(p["registers"]["block"], 9), -int(p["name"][3:])))
    time_driver = timer_candidates[0]["name"] if timer_candidates else None
    if paired:
        time_driver = {"cm7": "TIM5", "cm4": "TIM2"}[core["name"]]
        if not {"TIM2", "TIM5"}.issubset({p["name"] for p in timer_candidates}):
            raise ValueError("paired core metadata lacks a required TIM2/TIM5 timebase")
    patterns = dict(gpio=r"GPIO[A-Z]", uart=r"(?:USART|UART|LPUART)\d+", can=r"(?:FDCAN|CAN)\d*",
                    usb_cdc=r"USB.*", spi=r"SPI\d+", pwm=r"(?:TIM|LPTIM|HRTIM)\d+", timer=r"(?:TIM|LPTIM|HRTIM)\d+")
    kinds = dict(gpio={"gpio"}, uart={"usart"}, can={"can", "fdcan"}, usb_cdc={"usb", "otg"},
                 spi={"spi"}, pwm={"timer", "lptim", "hrtim"}, timer={"timer", "lptim", "hrtim"})
    result = {}
    for category in CATEGORIES:
        candidates = [p for p in peripherals if p.get("registers", {}).get("kind") in kinds[category]
                      or re.fullmatch(patterns[category], p["name"])]
        row = dict(status="not-present", hardware_present=bool(candidates), package=selected["name"],
                   package_type=selected["package"], core=core["name"], time_driver=time_driver,
                   candidate_instances=[p["name"] for p in candidates], clock_source=clock_source, clock_plan=clock_plan)
        result[category] = row
        if not candidates:
            row["reason"] = "no matching controller in exact core PAC metadata"
            continue
        row.update(status="unsupported", reason="no supported constructor/DMA/IRQ recipe")
        if time_driver is None:
            row.update(status="blocked", reason="no explicit supported HAL time-driver assignment")
            continue
        package_route = False
        for peripheral in candidates:
            name = peripheral["name"]
            registers = peripheral.get("registers", {})
            if registers.get("kind") not in kinds[category]:
                continue
            signals = {"uart": ["TX", "RX"], "can": ["RX", "TX"], "usb_cdc": ["DP", "DM"],
                       "spi": ["SCK", "MOSI", "MISO"], "pwm": ["CH1"]}.get(category, [])
            if category == "gpio":
                choices = [{"pins": {"OUT": {"pin": pin}}, "remap": None} for pin in sorted(bonded)
                           if pin.startswith("P" + name[-1]) and pin not in {"PA13", "PA14", "PB3", "PB4", "PA15"}]
            else:
                choices = list(pin_choices(peripheral, signals, bonded)) if signals else [{"pins": {}, "remap": None}]
            package_route |= bool(choices)
            if not choices:
                continue
            if category in {"pwm", "timer"} and (name in ({"TIM2", "TIM5"} if paired else {time_driver}) or registers.get("block") not in GENERAL_TIMERS or not irq(peripheral, "CC")):
                continue
            choice = choices[0]
            extra = {}
            if category == "uart":
                dma = dma_pair(core, peripheral)
                if dma is None or not irq(peripheral, "GLOBAL"):
                    continue
                extra = dict(dma=list(dma), interrupts={"GLOBAL": irq(peripheral, "GLOBAL")})
            elif category == "can":
                required = ["IT0", "IT1"] if fdcan(registers) else ["TX", "RX0", "RX1", "SCE"]
                if any(not irq(peripheral, s) for s in required):
                    continue
                extra["interrupts"] = {s: irq(peripheral, s) for s in required}
            elif category == "usb_cdc":
                interrupt = irq(peripheral, "GLOBAL") if registers["kind"] == "otg" else irq(peripheral, "LP") or irq(peripheral, "GLOBAL")
                if not interrupt:
                    continue
                extra["interrupts"] = {"GLOBAL": interrupt}
            elif category == "timer":
                extra["interrupts"] = {"CC": irq(peripheral, "CC")}
            elif category == "spi":
                spare = sorted(bonded - {p["pin"] for p in choice["pins"].values()} - {"PA13", "PA14", "PA15", "PB3", "PB4"})
                if not spare:
                    continue
                extra["chip_select"] = spare[0]
            if feature.startswith("stm32f1") and category in {"uart", "can", "spi", "pwm"}:
                try:
                    extra["afio_type"] = afio_type(peripheral, choice, afio_widths or {})
                except ValueError as error:
                    row["reason"] = str(error)
                    continue
            row.update(status="planned", reason="exact-package constructor recipe; build not yet attempted",
                       instance=name, registers=registers, **choice, **extra)
            row["source"] = source(category, row, paired=paired)
            break
        if row["status"] == "unsupported" and not package_route:
            row.update(status="package-unavailable", reason="controller exists but required signals are not jointly bonded in selected package/remap")
    return result


def source(category, row, *, paired=False):
    name = row["instance"]
    pins = {s: "p." + pin["pin"] for s, pin in row["pins"].items()}
    bindings = {}
    def bind(interrupt, handler):
        bindings.setdefault(interrupt, []).append(handler)
    def generic(count):
        return "::<" + ", ".join(["_"] * count + [row["afio_type"]]) + ">" if "afio_type" in row else ""
    extra = ""
    if category == "gpio":
        body = f"let mut object = hal::gpio::Output::new({pins['OUT']}, hal::gpio::Level::Low, hal::gpio::Speed::Low); object.set_low();"
    elif category == "uart":
        bind(row["interrupts"]["GLOBAL"], f"hal::usart::InterruptHandler<hal::peripherals::{name}>")
        for dma in row["dma"]:
            bind(dma["interrupt"], f"hal::dma::InterruptHandler<hal::peripherals::{dma['name']}>")
        tx, rx = ("p." + d["name"] for d in row["dma"])
        body = (f"let uart = hal::usart::Uart::new{generic(3)}(p.{name}, {pins['TX']}, {pins['RX']}, {tx}, {rx}, Irqs, Default::default()).unwrap();\n"
                "let mut object = embodied_stm32::uart::UartStream::new(uart);")
    elif category == "spi":
        body = (f"let bus = hal::spi::Spi::new_blocking{generic(1)}(p.{name}, {pins['SCK']}, {pins['MOSI']}, {pins['MISO']}, Default::default());\n"
                f"let cs = hal::gpio::Output::new(p.{row['chip_select']}, hal::gpio::Level::High, hal::gpio::Speed::Low);\n"
                "let mut object = embodied_stm32::bus::SpiBusDevice::new(bus, cs).unwrap();")
    elif category == "pwm":
        pin_generic = f"::<_, hal::timer::Ch1, {row['afio_type']}>" if "afio_type" in row else "::<_, hal::timer::Ch1>"
        body = (f"let pin = hal::timer::simple_pwm::PwmPin{pin_generic}::new({pins['CH1']}, hal::gpio::OutputType::PushPull);\n"
                f"let mut pwm = hal::timer::simple_pwm::SimplePwm::new{generic(0)}(p.{name}, Some(pin), None, None, None, hal::time::Hertz(1000), Default::default());\n"
                "let mut object = embodied_stm32::bus::PwmAdapter::new(pwm.ch1()); object.disable().unwrap();")
    elif category == "timer":
        extra = """use embodied_stm32::hardware_timer::{CompareState, CompareStateStorage, CompareInterruptHandler, Stm32CompareTimer};
struct Storage;
static STATE: CompareState = CompareState::new();
impl CompareStateStorage for Storage { fn state() -> &'static CompareState { &STATE } }
fn now() -> embodied_core::time::Instant { embodied_core::time::Instant::from_micros(embassy_time::Instant::now().as_micros()) }
"""
        bind(row["interrupts"]["CC"], f"CompareInterruptHandler<hal::peripherals::{name}, Storage>")
        body = f"let mut object = Stm32CompareTimer::<_, Storage, _>::new(p.{name}, hal::timer::Channel::Ch1, Irqs, 1_000_000, now).unwrap();"
    elif category == "can":
        if fdcan(row["registers"]):
            handlers = {"IT0": "IT0InterruptHandler", "IT1": "IT1InterruptHandler"}
            body = (f"let mut config = hal::can::CanConfigurator::new(p.{name}, {pins['RX']}, {pins['TX']}, Irqs);\n"
                    "config.set_bitrate(500_000);\n"
                    "let can = config.into_internal_loopback_mode();")
        else:
            handlers = {"TX": "TxInterruptHandler", "RX0": "Rx0InterruptHandler", "RX1": "Rx1InterruptHandler", "SCE": "SceInterruptHandler"}
            body = (f"let mut can = hal::can::Can::new{generic(1)}(p.{name}, {pins['RX']}, {pins['TX']}, Irqs);\n"
                    "can.modify_config().set_loopback(true).set_silent(true);")
        for signal, handler in handlers.items():
            bind(row["interrupts"][signal], f"hal::can::{handler}<hal::peripherals::{name}>")
        body += "\nlet mut object = embodied_stm32::can::CanAdapter::new(can);"
    elif category == "usb_cdc":
        bind(row["interrupts"]["GLOBAL"], f"hal::usb::InterruptHandler<hal::peripherals::{name}>")
        extra = """use static_cell::StaticCell;
static CONFIG: StaticCell<[u8; 256]> = StaticCell::new();
static BOS: StaticCell<[u8; 256]> = StaticCell::new();
static CONTROL: StaticCell<[u8; 64]> = StaticCell::new();
static RX: StaticCell<[u8; 64]> = StaticCell::new();
static STATE: StaticCell<embassy_usb::class::cdc_acm::State> = StaticCell::new();
"""
        if row["registers"]["kind"] == "otg":
            extra += "static EP: StaticCell<[u8; 256]> = StaticCell::new();\n"
            driver = f"hal::usb::Driver::new_fs(p.{name}, {pins['DP']}, {pins['DM']}, Irqs, EP.init([0;256]), Default::default())"
        else:
            driver = f"hal::usb::Driver::new(p.{name}, {pins['DP']}, {pins['DM']}, Irqs)"
        body = (f"let driver = {driver};\n"
                "let mut builder = embassy_usb::Builder::new(driver, embassy_usb::Config::new(0xc0de, 0xcafe), CONFIG.init([0;256]), BOS.init([0;256]), &mut [], CONTROL.init([0;64]));\n"
                "let class = embassy_usb::class::cdc_acm::CdcAcmClass::new(&mut builder, STATE.init(embassy_usb::class::cdc_acm::State::new()), 64);\n"
                "let mut device = builder.build(); core::hint::black_box(&mut device);\n"
                "let mut object = embodied_stm32::usb::split(class, RX.init([0;64]));")
    else:
        raise ValueError(category)
    binding_source = ""
    if bindings:
        binding_source = "hal::bind_interrupts!(struct Irqs {\n" + "\n".join(k + " => " + ", ".join(v) + ";" for k, v in sorted(bindings.items())) + "\n});\n"
    startup = "#[path = \"../dual_startup.rs\"]\nmod dual_startup;\n" if paired else ""
    init = "dual_startup::init()" if paired else "hal::init(probe_config())"
    clock_source = row.get("clock_source", DEFAULT_CLOCK_SOURCE)
    return ("#![no_std]\n#![no_main]\nuse embodied_stm32::hal;\nuse defmt_rtt as _;\n" + startup + extra + binding_source + clock_source
            + "#[panic_handler]\nfn panic(_: &core::panic::PanicInfo) -> ! { loop { cortex_m::asm::wfi(); } }\n"
            + f"#[cortex_m_rt::entry]\nfn main() -> ! {{ probe_constructor_{category}({init}) }}\n"
            + f"#[inline(never)]\n#[unsafe(no_mangle)]\npub fn probe_constructor_{category}(p: hal::Peripherals) -> ! {{\n{body}\n"
            + "core::hint::black_box(&mut object); loop { core::hint::black_box(&mut object); cortex_m::asm::wfi(); }\n}\n")
