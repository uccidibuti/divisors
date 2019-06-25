# Divisors
A blazing fast Rust library to find all divisors of a natural number. This library works with u8, u16, u32, u64, u128 and usize types.
## Example
``` Rust
extern crate divisors;
use std::time::{Instant};

fn main() {
    let n: u128 = 934832147123321;
    println!("finding divisors of {}", n);
    let start_time = Instant::now();
    let v = divisors::get_divisors(n);
    println!("time = {:?}, divisors = {:?}", start_time.elapsed(), v);
}
/* Output:
finding divisors of 934832147123321
time = 663.484µs, divisors = [19, 5011, 63397, 95209, 154877, 1204543, 2942663, 317682367, 776088647, 6035964973, 9818737169, 14745684293, 186556006211, 49201691953859]
*/
```

## Documentations
```sh
cargo doc --no-deps --open
```

## Benchmarks
```sh
cargo bench
```

## Test
```sh
cargo test
```

## Example
```sh
cargo run --release --example example
```
## License
MIT.
