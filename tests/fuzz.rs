#[cfg(test)]
extern crate rand;
extern crate bitvector;

use rand::Rng;
use rand::distributions::Uniform;
use bitvector::*;
use std::collections::HashSet;

#[test]
fn fuzz_test() {
    let mut rng = rand::thread_rng();
    for _ in 0 .. 5 {
        let mut v1 : Vec<usize> = rng.sample_iter(Uniform::new(1,10_000_000)).take(rng.gen_range(10,10_000)).collect();
        let mut v2 : Vec<usize> = rng.sample_iter(Uniform::new(1,10_000_000)).take(rng.gen_range(10,10_000)).collect();

        // reference hashset
        let r1 = v1.iter().cloned().collect::<HashSet<usize>>();
        let r2 = v2.iter().cloned().collect::<HashSet<usize>>();


        // bitvector
        let b1 = v1.iter().cloned().collect::<BitVector>();
        let b2 = v2.iter().cloned().collect::<BitVector>();

        // Step1. Check if collect::<BitVector>() works
        for i in 0 ..= b1.capacity() {
            assert_eq!(b1.contains(i), r1.contains(&i));
        }

        // STep2. Check if iter() works
        let mut v1_reconstruct = b1.iter().collect::<HashSet<usize>>();
        for i in 0 ..= b1.capacity() {
            assert_eq!(b1.contains(i), v1_reconstruct.contains(&i));
            assert_eq!(r1.contains(&i), v1_reconstruct.contains(&i));
        }


        let mut u_b = b1.union(&b2).iter().collect::<Vec<usize>>();
        let mut u_r = r1.union(&r2).cloned().collect::<Vec<usize>>();
        u_b.sort(); u_r.sort();
        for (i, j) in u_b.iter().zip(u_r.iter()) {
            assert_eq!(i,j);
        }

        assert_eq!(u_b, u_r);

    }
}