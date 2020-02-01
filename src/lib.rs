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
}