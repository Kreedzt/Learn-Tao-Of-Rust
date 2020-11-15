#![feature(arbitrary_self_types, futures_api, async_await, pin)]
// TODO: 无法编译
use futures::{
    executor::ThreadPool,
    task::{SpawnExt, Waker, Context},
};
use std::future::Future;
use std::pin::Pin;
use std::task::*;

pub struct AlmostReady {
    ready: bool,
    value: i32,
}

pub fn almost_ready(value: i32) -> AlmostReady {
    AlmostReady {
        ready: false,
        value,
    }
}

impl Future for AlmostReady {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, lw: &Waker) -> Poll<Self::Output> {
        if self.ready {
            Poll::Ready(self.value + 1)
        } else {
            unsafe {
                Pin::get_unchecked_mut(self).ready = true;
            }
            lw.wake();
            Poll::Pending
        }
    }
}

fn main() {
    let mut executor = ThreadPool::new().unwrap();

    let future = async {
        println!("howdy!");
        let x = almost_ready(5).await;
        println!("done: {:?}", x);
    };

    executor.run(future);
}
