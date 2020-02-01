#![feature(test)]
extern crate test;

pub mod executor;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use futures::Future;
    use futures::task::{Context, Poll};
    use std::pin::Pin;
    //use bastion_executor::prelude::*;
    use lightproc::proc_stack::ProcStack;

    struct Yields(u32);
    impl Future for Yields {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.0 == 0 {
                Poll::Ready(())
            } else {
                self.0 -= 1;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
    //STJEPANG
    #[bench]
    fn custom_block_on_1000_yields_stjepang(b: &mut Bencher) {
        b.iter(|| executor::stjepang::block_on(Yields(1000)));
    }
    #[bench]
    fn custom_block_on_100_yields_stjepang(b: &mut Bencher) {
        b.iter(|| executor::stjepang::block_on(Yields(100)));
    }
    #[bench]
    fn custom_block_on_10_yields_stjepang(b: &mut Bencher) {
        b.iter(|| executor::stjepang::block_on(Yields(10)));
    }
    //EXTREME
    #[bench]
    fn custom_block_on_1000_yields_extreme(b: &mut Bencher) {
        b.iter(|| executor::extreme::run(Yields(1000)));
    }
    #[bench]
    fn custom_block_on_100_yields_extreme(b: &mut Bencher) {
        b.iter(|| executor::extreme::run(Yields(100)));
    }
    #[bench]
    fn custom_block_on_10_yields_extreme(b: &mut Bencher) {
        b.iter(|| executor::extreme::run(Yields(10)));
    }
    //FUTURES
    #[bench]
    fn custom_block_on_1000_yields_futures(b: &mut Bencher) {
        b.iter(|| futures::executor::block_on(Yields(1000)));
    }
    #[bench]
    fn custom_block_on_100_yields_futures(b: &mut Bencher) {
        b.iter(|| futures::executor::block_on(Yields(100)));
    }
    #[bench]
    fn custom_block_on_10_yields_futures(b: &mut Bencher) {
        b.iter(|| futures::executor::block_on(Yields(10)));
    }
    // TOKIO
    #[bench]
    fn custom_block_on_1000_yields_tokio(b: &mut Bencher) {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| rt.block_on(Yields(1000)));
    }
    #[bench]
    fn custom_block_on_100_yields_tokio(b: &mut Bencher) {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| rt.block_on(Yields(100)));
    }
    #[bench]
    fn custom_block_on_10_yields_tokio(b: &mut Bencher) {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| rt.block_on(Yields(10)));
    }
    // ASYNC-STD
    #[bench]
    fn custom_block_on_1000_yields_async_std(b: &mut Bencher) {
        b.iter(|| async_std::task::block_on(Yields(1000)));
    }
    #[bench]
    fn custom_block_on_100_yields_async_std(b: &mut Bencher) {
        b.iter(|| async_std::task::block_on(Yields(100)));
    }
    #[bench]
    fn custom_block_on_10_yields_async_std(b: &mut Bencher) {
        b.iter(|| async_std::task::block_on(Yields(10)));
    }
    // BASTION EXECUTOR
    #[bench]
    fn custom_block_on_1000_yields_bastion(b: &mut Bencher) {
        let stack = ProcStack::default();
        b.iter(|| bastion_executor::run::run(Yields(1000), stack.clone()));
    }
    #[bench]
    fn custom_block_on_100_yields_bastion(b: &mut Bencher) {
        let stack = ProcStack::default();
        b.iter(|| bastion_executor::run::run(Yields(100), stack.clone()));
    }
    #[bench]
    fn custom_block_on_10_yields_bastion(b: &mut Bencher) {
        let stack = ProcStack::default();
        b.iter(|| bastion_executor::run::run(Yields(10), stack.clone()));
    }
}