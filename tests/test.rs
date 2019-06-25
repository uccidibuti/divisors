extern crate divisors;
extern crate rand;

use rand::prelude::*;

#[test]
fn test() {
    let mut rng = rand::thread_rng();
    
    for _i in 0..10000 {
        let n: u32 = rng.gen::<u32>();
        if n == 0 { continue; }
        let v0 = divisors::get_divisors(n);
        let v1 = get_divisors_standard(n);

        
        for i in 0..v0.len() {
            assert!(v0[i] == v1[i]);        
        }
    }
}

/// geeksforgeeks solution: https://www.geeksforgeeks.org/find-divisors-natural-number-set-1/
fn get_divisors_standard(n: u32) -> Vec<u32> {
    let mut v = Vec::new();
    let n_sqrt = (n as f32).sqrt() as u32 + 1;

    for i in 2..n_sqrt {
        if  n % i == 0 {
            if n / i == i {
                v.push(i);
            }
            else {
                v.push(i);
                v.push(n / i);
            }
        }
    }
    v.sort();
    v
}
