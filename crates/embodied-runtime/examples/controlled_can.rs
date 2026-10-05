//! Host-only demonstration of typed worker exits and retained-frame recovery.
//! The fake driver is polled explicitly; this is not an Embassy executor or HIL run.
use std::{
    cell::{Cell, RefCell},
    future::{Future, poll_fn},
    pin::Pin,
    task::{Context, Poll, Waker},
};

use embodied_core::{
    can::{CanFrame, Id},
    communication::{AsyncCanTx, CanError, CanTxOutcome},
};
use embodied_runtime::{
    can::CanTxService,
    control::{RunControl, RunExit},
};

struct FakeCan<'a> {
    ready: &'a Cell<bool>,
    accepted: &'a RefCell<Vec<CanFrame>>,
}

impl AsyncCanTx for FakeCan<'_> {
    fn transmit(
        &mut self,
        frame: &CanFrame,
    ) -> impl Future<Output = Result<CanTxOutcome, CanError>> {
        poll_fn(|_| {
            if self.ready.get() {
                self.accepted.borrow_mut().push(*frame);
                Poll::Ready(Ok(CanTxOutcome::default()))
            } else {
                Poll::Pending
            }
        })
    }
}

fn poll<F: Future>(future: Pin<&mut F>) -> Poll<F::Output> {
    future.poll(&mut Context::from_waker(Waker::noop()))
}

fn main() {
    let service = CanTxService::<2>::new();
    let first = CanFrame::new(Id::Standard(0x201), &[1]).unwrap();
    let second = CanFrame::new(Id::Standard(0x202), &[2]).unwrap();
    service.try_enqueue(first).unwrap();
    service.try_enqueue(second).unwrap();

    let ready = Cell::new(false);
    let accepted = RefCell::new(Vec::new());
    let mut driver = FakeCan {
        ready: &ready,
        accepted: &accepted,
    };

    let control = RunControl::new();
    {
        let mut worker = std::pin::pin!(service.run_controlled(&mut driver, &control));
        assert!(poll(worker.as_mut()).is_pending());
        control.request_stop();
        assert!(poll(worker.as_mut()).is_pending());
        ready.set(true);
        assert_eq!(poll(worker.as_mut()), Poll::Ready(Ok(RunExit::Stopped)));
    }
    assert_eq!(accepted.borrow().as_slice(), &[first]);
    assert_eq!(service.queued_len(), 1);
    println!("graceful stop: accepted=1, queued=1");

    // A new control is intentional: requests are latched, never reset in place.
    let control = RunControl::new();
    ready.set(false);
    {
        let mut worker = std::pin::pin!(service.run_controlled(&mut driver, &control));
        assert!(poll(worker.as_mut()).is_pending());
        control.request_abort();
        assert_eq!(poll(worker.as_mut()), Poll::Ready(Ok(RunExit::Aborted)));
    }
    assert_eq!(service.in_flight(), Some(second));
    assert_eq!(service.stats().sent, 1);
    println!("abort: pending frame retained, accepted=1");

    ready.set(true);
    {
        let mut retry = std::pin::pin!(service.send_next(&mut driver));
        assert_eq!(
            poll(retry.as_mut()),
            Poll::Ready(Ok(CanTxOutcome::default()))
        );
    }
    assert_eq!(accepted.borrow().as_slice(), &[first, second]);
    assert_eq!(service.in_flight(), None);
    assert_eq!(service.stats().sent, 2);
    println!("recovery: accepted=2, no duplicated frame");
}
