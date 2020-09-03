extern crate rand;

use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    // This may not be cryptographically safe, use
    // `OsRng` (for example) in production software.
    thread_rng().gen_range(2u64, p - 1)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modpow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modpow(b_pub, a, p)
}

// compute b^e % m
fn modpow(b: u64, e: u64, m: u64) -> u64 {
    (0..e).fold(1, |a, _| (a * b) % m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_key_in_range_key() {
        let primes: Vec<u64> = vec![
            5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 773, 967, 3461, 6131,
        ];
        let private_keys: Vec<u64> = primes.iter().map(|x| private_key(*x)).collect();
        for i in 0..primes.len() {
            assert!(1 < private_keys[i] && private_keys[i] < primes[i]);
        }
    }

    #[test]
    fn test_public_key_correct() {
        let p: u64 = 23;
        let g: u64 = 5;

        let private_key: u64 = 6;
        let expected: u64 = 8;
        assert_eq!(public_key(p, g, private_key), expected);
    }

    #[test]
    fn test_secret_key_correct() {
        let p: u64 = 11;
        let private_key_a: u64 = 7;
        let public_key_b: u64 = 8;
        let secret = secret(p, public_key_b, private_key_a);
        let expected: u64 = 2;
        assert_eq!(secret, expected);
    }

    #[test]
    fn test_public_ley_correct_big_numbers() {
        let p: u64 = 4_294_967_299;
        let g: u64 = 8;
        let private_key: u64 = 4_294_967_296;
        let expected: u64 = 4096;
        assert_eq!(public_key(p, g, private_key), expected);
    }

    #[test]
    fn test_changed_secret_key() {
        let p: u64 = 13;
        let g: u64 = 11;

        // just using `p` as an input for rng
        // could be any number
        let private_key_a = private_key(p);
        let private_key_b = private_key(p);

        // assert!(private_key_a != private_key_b);

        let public_key_a = public_key(p, g, private_key_a);
        let public_key_b = public_key(p, g, private_key_b);

        // Key exchange
        let secret_a = secret(p, public_key_b, private_key_a);
        let secret_b = secret(p, public_key_a, private_key_b);

        assert_eq!(secret_a, secret_b);
    }
}
