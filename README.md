# blocking_tests

Benchmarking of different block_on() implementations.
Inspired after https://stjepang.github.io/2020/01/25/build-your-own-block-on.html article.

```
test tests::custom_block_on_1000_yields_async_std ... bench:     587,249 ns/iter (+/- 33,765)
test tests::custom_block_on_1000_yields_extreme   ... bench:      40,935 ns/iter (+/- 1,357)
test tests::custom_block_on_1000_yields_futures   ... bench:      34,035 ns/iter (+/- 1,094)
test tests::custom_block_on_1000_yields_bastion   ... bench:      28,149 ns/iter (+/- 1,555)
test tests::custom_block_on_1000_yields_stjepang  ... bench:      20,398 ns/iter (+/- 913)
test tests::custom_block_on_1000_yields_tokio     ... bench:      16,722 ns/iter (+/- 526)

test tests::custom_block_on_100_yields_async_std  ... bench:      58,322 ns/iter (+/- 2,158)
test tests::custom_block_on_100_yields_extreme    ... bench:       4,225 ns/iter (+/- 179)
test tests::custom_block_on_100_yields_futures    ... bench:       3,415 ns/iter (+/- 76)
test tests::custom_block_on_100_yields_bastion    ... bench:       2,894 ns/iter (+/- 119)
test tests::custom_block_on_100_yields_stjepang   ... bench:       2,046 ns/iter (+/- 69)
test tests::custom_block_on_100_yields_tokio      ... bench:       1,693 ns/iter (+/- 84)

test tests::custom_block_on_10_yields_async_std   ... bench:       6,295 ns/iter (+/- 185)
test tests::custom_block_on_10_yields_extreme     ... bench:         606 ns/iter (+/- 30)
test tests::custom_block_on_10_yields_futures     ... bench:         353 ns/iter (+/- 58)
test tests::custom_block_on_10_yields_bastion     ... bench:         342 ns/iter (+/- 13)
test tests::custom_block_on_10_yields_tokio       ... bench:         245 ns/iter (+/- 4)
test tests::custom_block_on_10_yields_stjepang    ... bench:         209 ns/iter (+/- 6)

test tests::custom_block_on_0_yields_extreme      ... bench:         186 ns/iter (+/- 12)
test tests::custom_block_on_0_yields_async_std    ... bench:          45 ns/iter (+/- 2)
test tests::custom_block_on_0_yields_bastion      ... bench:          42 ns/iter (+/- 2)
test tests::custom_block_on_0_yields_futures      ... bench:          13 ns/iter (+/- 0)
test tests::custom_block_on_0_yields_tokio        ... bench:          13 ns/iter (+/- 0)
test tests::custom_block_on_0_yields_stjepang     ... bench:           5 ns/iter (+/- 0)
```
To try just run `cargo bench` after clone!
