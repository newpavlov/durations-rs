# Durations
This crate defines several `Duration` constants commonly (or rarely) used
in practice.

## Usage example
```Rust
extern crate durations;
 
use durations::{SECOND as S, MILLISECOND as MS};
 
std::thread::sleep(2*S + 200*MS);
// or alternatively
std::thread::sleep(2.2*S);
```

## License

All crates licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
