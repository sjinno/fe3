// // Without using a library...
// pub fn nth(n: u32) -> u32 {
//     let mut primes = vec![2, 3, 5, 7];
//     if n == 0 || n == 1 || n == 2 || n == 3 {
//         return primes[n as usize];
//     }

//     let mut count = 3;
//     let mut potential = 11;
//     while count != n {
//         let length = primes.len();
//         for i in 0..length {
//             let p = primes[i];
//             let rem = potential % p;
//             println!("p: {}. rem: {}.", p, rem);
//             if rem == 0 {
//                 potential += 2;
//                 break;
//             }

//             if i == length - 1 {
//                 primes.push(potential);
//                 count += 1;
//                 potential += 2;
//             }
//         }
//     }

//     return primes[n as usize];
// }

use concurrent_prime_sieve::collection;

pub fn nth(n: u32) -> u32 {
    if n > 1000 {
        let primes = collection::primes_concurrently(n as usize * 11, 8);
        return primes[n as usize] as u32;
    }
    let primes = collection::primes(n as usize * 4);
    primes[n as usize] as u32
}

// use std::ops::RangeFrom;

// pub fn nth(n: u32) -> Option<u32> {
//     (RangeFrom { start: 2 })
//         .filter(|x| is_prime(x))
//         .take(n as usize)
//         .last()
// }

// fn is_prime(n: &u32) -> bool {
//     !(2..*n).any(|x| n % x == 0)
// }
