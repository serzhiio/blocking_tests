# blocking_tests

Benchmarking of different block_on() implementations.
Inspired after https://stjepang.github.io/2020/01/25/build-your-own-block-on.html article.

```
running 15 tests
test tests::custom_block_on_1000_yields_async_std ... bench:     572,771 ns/iter (+/- 14,446)
test tests::custom_block_on_1000_yields_extreme   ... bench:      41,377 ns/iter (+/- 3,930)
test tests::custom_block_on_1000_yields_futures   ... bench:      30,769 ns/iter (+/- 832)
test tests::custom_block_on_1000_yields_bastion   ... bench:      28,881 ns/iter (+/- 921)
test tests::custom_block_on_1000_yields_stjepang  ... bench:      20,615 ns/iter (+/- 910)
test tests::custom_block_on_1000_yields_tokio     ... bench:      16,722 ns/iter (+/- 397)

test tests::custom_block_on_100_yields_async_std  ... bench:      57,251 ns/iter (+/- 2,238)
test tests::custom_block_on_100_yields_extreme    ... bench:       4,305 ns/iter (+/- 1,090)
test tests::custom_block_on_100_yields_futures    ... bench:       3,085 ns/iter (+/- 177)
test tests::custom_block_on_100_yields_bastion    ... bench:       2,962 ns/iter (+/- 82)
test tests::custom_block_on_100_yields_stjepang   ... bench:       2,065 ns/iter (+/- 43)
test tests::custom_block_on_100_yields_tokio      ... bench:       1,680 ns/iter (+/- 45)

test tests::custom_block_on_10_yields_async_std   ... bench:       6,160 ns/iter (+/- 299)
test tests::custom_block_on_10_yields_extreme     ... bench:         622 ns/iter (+/- 18)
test tests::custom_block_on_10_yields_bastion     ... bench:         347 ns/iter (+/- 15)
test tests::custom_block_on_10_yields_futures     ... bench:         317 ns/iter (+/- 5)
test tests::custom_block_on_10_yields_stjepang    ... bench:         209 ns/iter (+/- 10)
test tests::custom_block_on_10_yields_tokio       ... bench:         175 ns/iter (+/- 4)

```

To try just run `cargo bench` after clone!
