# blocking_tests

Benchmarking of different block_on() implementations.
Inspired after https://stjepang.github.io/2020/01/25/build-your-own-block-on.html article.

```
running 15 tests
test tests::custom_block_on_1000_yields_async_std ... bench:     569,276 ns/iter (+/- 7,220)
test tests::custom_block_on_1000_yields_extreme   ... bench:      41,077 ns/iter (+/- 2,903)
test tests::custom_block_on_1000_yields_futures   ... bench:      31,125 ns/iter (+/- 1,467)
test tests::custom_block_on_1000_yields_stjepang  ... bench:      20,659 ns/iter (+/- 558)
test tests::custom_block_on_1000_yields_tokio     ... bench:      16,764 ns/iter (+/- 548)

test tests::custom_block_on_100_yields_async_std  ... bench:      57,352 ns/iter (+/- 1,157)
test tests::custom_block_on_100_yields_extreme    ... bench:       4,362 ns/iter (+/- 415)
test tests::custom_block_on_100_yields_futures    ... bench:       3,073 ns/iter (+/- 276)
test tests::custom_block_on_100_yields_stjepang   ... bench:       2,058 ns/iter (+/- 148)
test tests::custom_block_on_100_yields_tokio      ... bench:       1,689 ns/iter (+/- 137)

test tests::custom_block_on_10_yields_async_std   ... bench:       6,147 ns/iter (+/- 184)
test tests::custom_block_on_10_yields_extreme     ... bench:         601 ns/iter (+/- 19)
test tests::custom_block_on_10_yields_futures     ... bench:         318 ns/iter (+/- 9)
test tests::custom_block_on_10_yields_stjepang    ... bench:         210 ns/iter (+/- 6)
test tests::custom_block_on_10_yields_tokio       ... bench:         174 ns/iter (+/- 14)
```

To try just run `cargo bench` after clone!
