use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use crate::support::waker_utils::simple_waker_into_waker;

pub fn async_vec() {
    let mut value = async { vec![1, 2, 3] };
    let pin_value = unsafe { Pin::new_unchecked(&mut value) };
    let waker = simple_waker_into_waker(&());
    let res = pin_value.poll(&mut Context::from_waker(&waker));
    let Poll::Ready(value) = res else {
        panic!("Future not ready");
    };
    println!("{:?}", value);
}
