//! Exclusive hardware compare timer with a chip-specific HAL implementation.
#![cfg(any(test, all(feature = "hal", stm32_has_timer)))]

use core::{
    cell::RefCell,
    task::{Context, Poll, Waker},
};
use critical_section::Mutex;
use embodied_core::time::Instant;
use embodied_runtime::hardware_timer::HardwareTimerChannel;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompareError {
    InvalidFrequency,
    DeadlineTooFar,
    StateInUse,
    GenerationExhausted,
    NotArmed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CounterWidth {
    Bits16,
    Bits32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TickRate {
    kernel_hz: u32,
    prescaler: u16,
    max_ticks: u32,
}

impl TickRate {
    fn new(kernel_hz: u32, ceiling_hz: u32, width: CounterWidth) -> Result<Self, CompareError> {
        if kernel_hz == 0 || ceiling_hz == 0 {
            return Err(CompareError::InvalidFrequency);
        }
        let divider = kernel_hz.div_ceil(ceiling_hz);
        let prescaler = u16::try_from(divider - 1).map_err(|_| CompareError::InvalidFrequency)?;
        Ok(Self {
            kernel_hz,
            prescaler,
            max_ticks: match width {
                CounterWidth::Bits16 => u16::MAX.into(),
                CounterWidth::Bits32 => u32::MAX,
            },
        })
    }
    fn ticks(self, micros: u64) -> Result<u32, CompareError> {
        let numerator = u128::from(micros) * u128::from(self.kernel_hz);
        let denominator = (u128::from(self.prescaler) + 1) * 1_000_000;
        let ticks = numerator.div_ceil(denominator);
        if ticks > u128::from(self.max_ticks) {
            Err(CompareError::DeadlineTooFar)
        } else {
            Ok(ticks as u32)
        }
    }
    fn max_delay_micros(self) -> u64 {
        // divider <= kernel_hz, so the final value is <= u32::MAX * 1_000_000.
        (u128::from(self.max_ticks) * (u128::from(self.prescaler) + 1) * 1_000_000
            / u128::from(self.kernel_hz)) as u64
    }
}

struct State {
    owner: Option<usize>,
    channel: usize,
    generation: u64,
    armed: bool,
    fired: Option<u64>,
    waker: Option<Waker>,
}

/// Static IRQ state. One live timer may claim it; generations survive releases.
pub struct CompareState {
    inner: Mutex<RefCell<State>>,
}
impl Default for CompareState {
    fn default() -> Self {
        Self::new()
    }
}
impl CompareState {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(RefCell::new(State {
                owner: None,
                channel: 0,
                generation: 0,
                armed: false,
                fired: None,
                waker: None,
            })),
        }
    }
}

/// An IRQ binding's storage provider must return the same static state on every
/// call. Use a distinct provider/state for each concurrently owned timer.
pub trait CompareStateStorage: 'static {
    fn state() -> &'static CompareState;
}

trait InterruptRegisters {
    fn disable(&self);
    fn stop(&self);
    fn clear(&self);
    /// Both this source's enable bit and status flag must be set.
    fn pending(&self) -> bool;
}
trait AlarmRegisters: InterruptRegisters {
    fn owner(&self) -> usize;
    fn channel(&self) -> usize {
        0
    }
    fn program(&self, ticks: u32);
    fn enable(&self);
    fn start(&self);
}
fn disarm(registers: &impl InterruptRegisters) {
    registers.disable();
    registers.stop();
    registers.clear();
}
fn complete(state: &mut State, registers: &impl InterruptRegisters) -> Option<Waker> {
    disarm(registers);
    state.armed = false;
    state.fired = Some(state.generation);
    state.waker.take()
}
fn interrupt<R: InterruptRegisters>(
    state: &CompareState,
    owner: usize,
    registers: impl FnOnce(usize) -> R,
) {
    let wake = critical_section::with(|cs| {
        let mut state = state.inner.borrow(cs).borrow_mut();
        // Do not touch a clock-gated peripheral after its owner was dropped.
        if state.owner != Some(owner) || !state.armed {
            return None;
        }
        let registers = registers(state.channel);
        if registers.pending() {
            complete(&mut state, &registers)
        } else {
            None
        }
    });
    if let Some(waker) = wake {
        waker.wake();
    }
}

struct Engine<B: AlarmRegisters, F: Fn() -> Instant> {
    registers: B,
    rate: TickRate,
    state: &'static CompareState,
    now: F,
}
impl<B: AlarmRegisters, F: Fn() -> Instant> Engine<B, F> {
    fn new(
        registers: B,
        rate: TickRate,
        state: &'static CompareState,
        now: F,
    ) -> Result<Self, CompareError> {
        critical_section::with(|cs| {
            let mut inner = state.inner.borrow(cs).borrow_mut();
            if inner.owner.is_some() {
                return Err(CompareError::StateInUse);
            }
            inner.owner = Some(registers.owner());
            inner.channel = registers.channel();
            Ok(())
        })?;
        Ok(Self {
            registers,
            rate,
            state,
            now,
        })
    }
}
impl<B: AlarmRegisters, F: Fn() -> Instant> HardwareTimerChannel for Engine<B, F> {
    type Error = CompareError;
    fn arm(&mut self, deadline: Instant) -> Result<(), CompareError> {
        self.cancel();
        // Sample after stopping the timer. Configuration/start latency makes an
        // alarm late, never early; fractional counter periods round upward.
        let remaining = deadline
            .as_micros()
            .saturating_sub((self.now)().as_micros());
        let ticks = self.rate.ticks(remaining)?;
        critical_section::with(|cs| {
            let mut state = self.state.inner.borrow(cs).borrow_mut();
            state.generation = state
                .generation
                .checked_add(1)
                .ok_or(CompareError::GenerationExhausted)?;
            if ticks == 0 {
                state.fired = Some(state.generation);
                return Ok(());
            }
            self.registers.program(ticks);
            self.registers.clear();
            state.armed = true;
            self.registers.enable();
            self.registers.start();
            Ok(())
        })
    }
    fn poll_expired(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), CompareError>> {
        let mut incoming = Some(cx.waker().clone());
        let (result, old) = critical_section::with(|cs| {
            let mut state = self.state.inner.borrow(cs).borrow_mut();
            if state.fired == Some(state.generation) {
                return (Poll::Ready(Ok(())), None);
            }
            if !state.armed {
                return (Poll::Ready(Err(CompareError::NotArmed)), None);
            }
            if self.registers.pending() {
                return (Poll::Ready(Ok(())), complete(&mut state, &self.registers));
            }
            let old = if state
                .waker
                .as_ref()
                .is_some_and(|old| old.will_wake(cx.waker()))
            {
                None
            } else {
                core::mem::replace(&mut state.waker, incoming.take())
            };
            (Poll::Pending, old)
        });
        drop(old);
        drop(incoming);
        result
    }
    fn cancel(&mut self) {
        let old = critical_section::with(|cs| {
            let mut state = self.state.inner.borrow(cs).borrow_mut();
            disarm(&self.registers);
            state.armed = false;
            state.fired = None;
            state.waker.take()
        });
        drop(old);
    }
}
impl<B: AlarmRegisters, F: Fn() -> Instant> Drop for Engine<B, F> {
    fn drop(&mut self) {
        self.cancel();
        critical_section::with(|cs| self.state.inner.borrow(cs).borrow_mut().owner = None);
    }
}

#[cfg(all(feature = "hal", stm32_has_timer))]
#[allow(unsafe_code)]
mod stm32 {
    use super::*;
    use core::marker::PhantomData;
    use embassy_stm32::{
        Peri,
        interrupt::typelevel::{Binding, Handler, Interrupt},
        timer::{
            Channel, GeneralInstance4Channel, TimerBits,
            low_level::{CountingMode, OutputCompareMode, Timer},
        },
    };

    struct Registers<'d, T: GeneralInstance4Channel> {
        timer: Timer<'d, T>,
        channel: Channel,
    }
    impl<T: GeneralInstance4Channel> InterruptRegisters for Registers<'_, T> {
        fn disable(&self) {
            self.timer.enable_input_interrupt(self.channel, false);
        }
        fn stop(&self) {
            self.timer.stop();
        }
        fn clear(&self) {
            self.timer.clear_input_interrupt(self.channel);
        }
        fn pending(&self) -> bool {
            self.timer
                .regs_gp16()
                .dier()
                .read()
                .ccie(self.channel.index())
                && self.timer.get_input_interrupt(self.channel)
        }
    }
    impl<T: GeneralInstance4Channel> AlarmRegisters for Registers<'_, T> {
        fn owner(&self) -> usize {
            T::regs() as usize
        }
        fn channel(&self) -> usize {
            self.channel.index()
        }
        fn program(&self, ticks: u32) {
            self.timer.reset();
            self.timer.generate_update_event();
            self.timer.set_compare_value(
                self.channel,
                match T::Word::try_from(ticks) {
                    Ok(word) => word,
                    Err(_) => unreachable!("ticks checked against timer width"),
                },
            );
        }
        fn enable(&self) {
            self.timer.enable_input_interrupt(self.channel, true);
        }
        fn start(&self) {
            self.timer.start();
        }
    }

    struct IrqRegisters<T: GeneralInstance4Channel> {
        channel: usize,
        _timer: PhantomData<T>,
    }
    impl<T: GeneralInstance4Channel> IrqRegisters<T> {
        fn regs(&self) -> embassy_stm32::pac::timer::TimGp16 {
            // SAFETY: GeneralInstance4Channel is sealed by Embassy to supported
            // TIM register blocks. The GP16 view is their documented common
            // control/status prefix, also used by Embassy's own CC IRQ handler.
            // All caller accesses happen under the same CS as the live owner.
            unsafe { embassy_stm32::pac::timer::TimGp16::from_ptr(T::regs()) }
        }
    }
    impl<T: GeneralInstance4Channel> InterruptRegisters for IrqRegisters<T> {
        fn disable(&self) {
            self.regs()
                .dier()
                .modify(|r| r.set_ccie(self.channel, false));
        }
        fn stop(&self) {
            self.regs().cr1().modify(|r| r.set_cen(false));
        }
        fn clear(&self) {
            self.regs().sr().write(|r| {
                r.0 = u32::MAX;
                r.set_ccif(self.channel, false);
            });
        }
        fn pending(&self) -> bool {
            self.regs().dier().read().ccie(self.channel)
                && self.regs().sr().read().ccif(self.channel)
        }
    }

    /// Bind the timer's CaptureCompareInterrupt to this handler using Embassy's
    /// bind_interrupts!. Include other handlers if the hardware vector is shared.
    pub struct CompareInterruptHandler<T: GeneralInstance4Channel, S: CompareStateStorage>(
        PhantomData<(T, S)>,
    );
    impl<T: GeneralInstance4Channel, S: CompareStateStorage> Handler<T::CaptureCompareInterrupt>
        for CompareInterruptHandler<T, S>
    {
        // SAFETY: This is the signature required by Embassy's IRQ contract. All
        // accesses are delegated to the owner/armed-gated critical-section path;
        // the handler does not fabricate a peripheral owner or borrow a driver.
        unsafe fn on_interrupt() {
            let state = S::state();
            // Owner, channel and flag are read within one CS, excluding rearm
            // and avoiding register access after the peripheral clock is gated.
            interrupt(state, T::regs() as usize, |channel| IrqRegisters::<T> {
                channel,
                _timer: PhantomData,
            });
        }
    }

    /// Owns the entire timer and one selected compare channel. It cannot share T
    /// with PWM, input capture, or the Embassy time driver. No output pin is used.
    ///
    /// Every arm restarts a one-pulse counter at zero. Deadlines longer than
    /// max_delay_micros() fail explicitly; there is no ambiguous wrapped deadline.
    /// `now` must return monotonic microseconds in the same domain as the deadline.
    /// RCC clocks must remain unchanged and running for the lifetime of the driver;
    /// stop/sleep modes that gate this timer are not supported by this driver.
    pub struct Stm32CompareTimer<
        'd,
        T: GeneralInstance4Channel,
        S: CompareStateStorage,
        F: Fn() -> Instant,
    > {
        engine: Engine<Registers<'d, T>, F>,
        _state: PhantomData<S>,
    }
    impl<'d, T: GeneralInstance4Channel, S: CompareStateStorage, F: Fn() -> Instant>
        Stm32CompareTimer<'d, T, S, F>
    {
        /// ceiling_hz controls the prescaler and maximum one-shot delay. The actual
        /// rate is kernel_hz/(PSC+1); conversion uses that exact rational rate.
        pub fn new(
            tim: Peri<'d, T>,
            channel: Channel,
            _irq: impl Binding<T::CaptureCompareInterrupt, CompareInterruptHandler<T, S>> + 'd,
            ceiling_hz: u32,
            now: F,
        ) -> Result<Self, CompareError> {
            let timer = Timer::new(tim);
            let width = match timer.bits() {
                TimerBits::Bits16 => CounterWidth::Bits16,
                TimerBits::Bits32 => CounterWidth::Bits32,
            };
            let rate = TickRate::new(timer.get_clock_frequency().0, ceiling_hz, width)?;
            timer.stop();
            // T is owned in its entirety; no other TIM source may be enabled.
            // Set clock/mode/output controls explicitly even if a bootloader left
            // the RCC enable bit set and Embassy therefore skipped RCC reset.
            timer.regs_gp16().dier().write(|r| r.0 = 0);
            timer.regs_gp16().ccer().write(|r| r.0 = 0);
            timer.regs_gp16().smcr().write(|r| r.0 = 0);
            timer.regs_gp16().cr2().write(|r| r.0 = 0);
            timer.regs_core().cr1().write(|r| r.set_opm(true));
            timer
                .regs_gp16()
                .ccmr_output(channel.index() / 2)
                .write(|r| r.0 = 0);
            timer.set_counting_mode(CountingMode::EdgeAlignedUp);
            timer.regs_core().psc().write_value(rate.prescaler);
            timer.set_autoreload_preload(false);
            timer.set_max_compare_value(match T::Word::try_from(rate.max_ticks) {
                Ok(word) => word,
                Err(_) => unreachable!("counter width matches T::Word"),
            });
            timer.set_output_compare_mode(channel, OutputCompareMode::Frozen);
            timer.set_output_compare_preload(channel, false);
            timer.enable_channel(channel, false);
            let engine = Engine::new(Registers { timer, channel }, rate, S::state(), now)?;
            // SAFETY: Binding proves this vector calls CompareInterruptHandler<T,S>.
            // The peripheral is exclusively owned, state initialized, and its IRQ
            // sources disabled. Never disable or unpend a potentially shared NVIC
            // vector: only this timer's CCIE/CCIF is touched by cancel/Drop.
            unsafe {
                T::CaptureCompareInterrupt::enable();
            }
            Ok(Self {
                engine,
                _state: PhantomData,
            })
        }
        pub fn max_delay_micros(&self) -> u64 {
            self.engine.rate.max_delay_micros()
        }
        pub fn prescaler(&self) -> u16 {
            self.engine.rate.prescaler
        }
        pub fn kernel_hz(&self) -> u32 {
            self.engine.rate.kernel_hz
        }
    }
    impl<T: GeneralInstance4Channel, S: CompareStateStorage, F: Fn() -> Instant>
        HardwareTimerChannel for Stm32CompareTimer<'_, T, S, F>
    {
        type Error = CompareError;
        fn arm(&mut self, deadline: Instant) -> Result<(), CompareError> {
            self.engine.arm(deadline)
        }
        fn poll_expired(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), CompareError>> {
            self.engine.poll_expired(cx)
        }
        fn cancel(&mut self) {
            self.engine.cancel();
        }
    }
}

#[cfg(all(feature = "hal", stm32_has_timer))]
pub use stm32::{CompareInterruptHandler, Stm32CompareTimer};
