# blocking_tests

Benchmarking of different block_on() implementations.
Inspired after https://stjepang.github.io/2020/01/25/build-your-own-block-on.html article.

```
test tests::custom_block_on_1000_yields_futures   ... bench:      31,203 ns/iter (+/- 569)
test tests::custom_block_on_1000_yields_extreme   ... bench:      41,229 ns/iter (+/- 884)
test tests::custom_block_on_1000_yields_bastion   ... bench:      28,425 ns/iter (+/- 1,152)
test tests::custom_block_on_1000_yields_async_std ... bench:      28,316 ns/iter (+/- 1,155)
test tests::custom_block_on_1000_yields_stjepang  ... bench:      20,967 ns/iter (+/- 511)
test tests::custom_block_on_1000_yields_tokio     ... bench:      17,035 ns/iter (+/- 404)

test tests::custom_block_on_100_yields_extreme    ... bench:       4,316 ns/iter (+/- 191)
test tests::custom_block_on_100_yields_futures    ... bench:       3,126 ns/iter (+/- 79)
test tests::custom_block_on_100_yields_async_std  ... bench:       2,893 ns/iter (+/- 98)
test tests::custom_block_on_100_yields_bastion    ... bench:       2,884 ns/iter (+/- 52)
test tests::custom_block_on_100_yields_stjepang   ... bench:       2,067 ns/iter (+/- 34)
test tests::custom_block_on_100_yields_tokio      ... bench:       1,681 ns/iter (+/- 40)

test tests::custom_block_on_10_yields_extreme     ... bench:         609 ns/iter (+/- 11)
test tests::custom_block_on_10_yields_bastion     ... bench:         343 ns/iter (+/- 7)
test tests::custom_block_on_10_yields_async_std   ... bench:         327 ns/iter (+/- 8)
test tests::custom_block_on_10_yields_futures     ... bench:         321 ns/iter (+/- 3)
test tests::custom_block_on_10_yields_stjepang    ... bench:         210 ns/iter (+/- 22)
test tests::custom_block_on_10_yields_tokio       ... bench:         175 ns/iter (+/- 4)

test tests::custom_block_on_0_yields_extreme      ... bench:         192 ns/iter (+/- 12)
test tests::custom_block_on_0_yields_bastion      ... bench:          43 ns/iter (+/- 2)
test tests::custom_block_on_0_yields_async_std    ... bench:          36 ns/iter (+/- 3)
test tests::custom_block_on_0_yields_futures      ... bench:          13 ns/iter (+/- 0)
test tests::custom_block_on_0_yields_tokio        ... bench:          12 ns/iter (+/- 0)
test tests::custom_block_on_0_yields_stjepang     ... bench:           6 ns/iter (+/- 0)
```
To try just run `cargo bench` after clone!
