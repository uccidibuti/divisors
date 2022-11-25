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
/// use std::time::Instant;
///
/// let n: u128 = 934832147123321;
/// println!("finding divisors of {}", n);
/// let start_time = Instant::now();
/// let v = divisors::get_divisors(n);
/// println!("time = {:?}, divisors = {:?}", start_time.elapsed(), v);
/// ```
/// 
/// # Panics
/// 
/// A panic could happen if number 2 or 3 cannot be converted to T.
pub fn get_divisors<T: Num>(n: T) -> Vec<T> {
    let zero = T::zero();
    let one = T::one();
    let two = T::from(2).unwrap();
    let mut number = n;
    let mut v = Vec::new();

    let mut count_divisors_2: usize = 0;
    while number & one == zero {
        v.push(two << count_divisors_2);
        count_divisors_2 += 1;
        number = number >> 1;
    }

    let mut x = T::from(3).unwrap();
    let mut number_sqrt = approximated_sqrt(number);
    while x < number_sqrt {
        let mut pow_x = x;
        let v_len = v.len();

        let x_is_a_divisors = number % x == zero;
        let mut pow_x_is_a_divisors = x_is_a_divisors;
        while pow_x_is_a_divisors {
            number = number.div(x);
            v.push(pow_x);
            push_new_divisors(&mut v, v_len, pow_x);
            pow_x_is_a_divisors = number % x == zero;
            if pow_x_is_a_divisors {
                pow_x = pow_x.mul(x);
            }
        }
        if x_is_a_divisors {
            number_sqrt = approximated_sqrt(number);
        }
        x = x + two;
    }

    if number > one && number != n {
        let v_len = v.len();
        v.push(number);
        push_new_divisors(&mut v, v_len, number);
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
/// let n: u32 = 63;
/// let n_sqrt  = divisors::approximated_sqrt(n);
/// assert_eq!(n_sqrt, 8);
/// ```
pub fn approximated_sqrt<T: Num>(n: T) -> T {
    let zero: T = T::zero();
    let one: T = T::one();
    let mut num_bits = (std::mem::size_of::<T>() << 3) - 1;
    while ((n >> num_bits) & one) == zero {
        num_bits -= 1;
    }

    one << ((num_bits >> 1) + 1)
}

/// Iterate on `v` from `0` to `v_len` and for each element `e`: push `e * x` in `v`.
fn push_new_divisors<T: Num>(v: &mut Vec<T>, v_len: usize, x: T) {
    for i in 0..v_len {
        v.push(x.mul(unsafe { *v.get_unchecked(i) }));
    }
}
