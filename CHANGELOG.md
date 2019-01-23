# 0.3.0 - 2019-01-23

* Bump minimum required rustc version to 1.22.0
* Fixed serde deserialization into owned string that previously caused panics
  when doing round-trip (de)serialization
* `HashEngine::block_size()` and `Hash::len()` are now associated constants
  `HashEngine::BLOCK_SIZE` and `Hash::LEN`
* Removed `block_size()` method from `Hash` trait. It is still available as
  `<T as Hash>::Engine::BLOCK_SIZE`

# 0.2.0 - 2019-01-15

* Add a constant-time comparison function
* Simplify `io::Write::write` implementations by having them do only partial writes
* Add fuzzing support
* Allow `Hash`es to be borrowed as `[u8]`
* Replace public `Hash` inners with `into_inner` method

# 0.1.0 - 2018-12-08

* Initial release

