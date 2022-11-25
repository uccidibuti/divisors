//! # divisors
//!
//! divisors is a blazing fast library to get all divisors of a natural number.

use num::{NumCast, PrimInt, Unsigned};
use std::fmt::Display;

pub trait Num: NumCast + Unsigned + Copy + PrimInt + Display {}

impl Num for u8 {}
impl Num for u16 {}
impl Num for u32 {}
impl Num for u64 {}
impl Num for u128 {}
impl Num for usize {}

/// Return a vector with all divisors ordered of n in range (1, n-1).
///
/// # Example
///
/// ```
/// use std::time::{Instant};
///
/// fn main() {
///     let n: u128 = 934832147123321;
///     println!("finding divisors of {}", n);
///     let start_time = Instant::now();
///     let v = divisors::get_divisors(n);
///     println!("time = {:?}, divisors = {:?}", start_time.elapsed(), v);
/// }
/// ```
pub fn get_divisors<T: Num>(n: T) -> Vec<T> {
    let _0: T = T::zero();
    let _1: T = T::one();
    let _2: T = T::from(2).unwrap();
    let mut _n = n;
    let mut v: Vec<T> = Vec::new();

    let mut count_divisors_2: usize = 0;
    while _n & _1 == _0 {
        v.push(_2 << count_divisors_2);
        count_divisors_2 += 1;
        _n = _n >> 1;
    }

    let mut _x: T = T::from(3).unwrap();
    let mut _n_sqrt: T = approximated_sqrt(_n);
    while _x < _n_sqrt {
        let mut _pow_x = _x;
        let v_len = v.len();
        let mut x_is_a_divisors = false;

        let mut pow_x_is_a_divisors = _n % _x == _0;
        while pow_x_is_a_divisors == true {
            _n = _n.div(_x);
            v.push(_pow_x);
            push_new_divisors(&mut v, v_len, _pow_x);
            pow_x_is_a_divisors = _n % _x == _0;
            if pow_x_is_a_divisors == true {
                _pow_x = _pow_x.mul(_x);
            }
            x_is_a_divisors = true;
        }
        _x = _x + _2;
        if x_is_a_divisors == true {
            _n_sqrt = approximated_sqrt(_n);
        }
    }

    if _n > _1 && _n != n {
        let v_len = v.len();
        v.push(_n);
        push_new_divisors(&mut v, v_len, _n);
    }

    if v.len() > 1 {
        v.sort();
        v.pop();
    }

    v
}

/// Return a number near and greather then sqrt(n).
///
/// # Example
///
/// ```
/// fn main() {
///     let n: u32 = 63;
///     let n_sqrt  = divisors::approximated_sqrt(n);
///     assert_eq!(n_sqrt, 8);
/// }
/// ```
pub fn approximated_sqrt<T: Num>(n: T) -> T {
    let _0: T = T::zero();
    let _1: T = T::one();
    let mut num_bits = (std::mem::size_of::<T>() << 3) - 1;
    while ((n >> num_bits) & _1) == _0 {
        num_bits -= 1;
    }

    _1 << ((num_bits >> 1) + 1)
}

/// Iterate on v from 0 to v_len and for each element e: push (e * _x) in v.
fn push_new_divisors<T: Num>(v: &mut Vec<T>, v_len: usize, _x: T) {
    for i in 0..v_len {
        v.push(_x.mul(unsafe { *v.get_unchecked(i) }));
    }
}
