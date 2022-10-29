use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use crate::support::drop_helpers::forget_and_drop;
use crate::support::drop_helpers::forget_and_no_inline_drop;
use crate::support::waker_utils::simple_waker_into_waker;

fn drop_helpers_vec_u64_no_inline() {
    let value: Vec<u64> = vec![1, 2, 3];
    forget_and_no_inline_drop(value);
}

fn drop_helpers_vec_u64() {
    let value: Vec<u64> = vec![1, 2, 3];
    forget_and_drop(value);
}

pub fn drop_helpers_async_vec_i32() {
    let mut value = async { vec![1, 2, 3] };
    let pin_value = unsafe { Pin::new_unchecked(&mut value) };
    let waker = simple_waker_into_waker(&());
    let res = pin_value.poll(&mut Context::from_waker(&waker));
    match res {
        Poll::Ready(value) => println!("{:?}", value),
        Poll::Pending => println!("Still pending"),
    }
    forget_and_no_inline_drop(value);
}
