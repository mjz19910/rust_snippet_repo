use std::task::{RawWaker, RawWakerVTable, Waker};

const VTABLE: RawWakerVTable =
    unsafe { RawWakerVTable::new(|s| simple_waker_clone(&*s), |_s| (), |_s| (), |_s| ()) };

fn simple_waker_clone(s: &()) -> RawWaker {
    RawWaker::new(s, &VTABLE)
}

pub fn simple_waker_into_waker(s: &()) -> Waker {
    let raw_waker = RawWaker::new(s, &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}
