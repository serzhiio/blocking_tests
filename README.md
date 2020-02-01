# blocking_tests

Benchmarking of different block_on() implementations by stjepang and by extreme.
Inspired after https://stjepang.github.io/2020/01/25/build-your-own-block-on.html article.

running 6 tests

`test tests::custom_block_on_1000_yields_extreme  ... bench:      42,287 ns/iter (+/- 1,216)
test tests::custom_block_on_1000_yields_stjepang ... bench:      20,631 ns/iter (+/- 306)
test tests::custom_block_on_100_yields_extreme   ... bench:       4,413 ns/iter (+/- 165)
test tests::custom_block_on_100_yields_stjepang  ... bench:       2,069 ns/iter (+/- 71)
test tests::custom_block_on_10_yields_extreme    ... bench:         612 ns/iter (+/- 18)
test tests::custom_block_on_10_yields_stjepang   ... bench:         209 ns/iter (+/- 4)`

To try just run `cargo bench` after clone!
