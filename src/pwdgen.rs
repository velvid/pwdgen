use constcat::concat;
use rand::seq::{IteratorRandom, SliceRandom};

// Sample character sets to use for pools.
pub const UPPER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWER: &'static str = "abcdefghijklmnopqrstuvwxyz";
pub const ALPHA: &'static str = concat!(UPPER, LOWER);
pub const NUMERIC: &'static str = "0123456789";
pub const SPECIAL: &'static str = "!@#$%^&*()_+-=[]{};':\",./<>?\\|";

pub struct Pool {
    pub name: Option<&'static str>,
    pub minimum: usize,
    pub char_set: &'static str,
}

pub fn from_pools<R>(rng: &mut R, min_length: usize, pools: &[Pool]) -> String
where
    R: rand::Rng + ?Sized,
{
    // If pools is empty, return an empty string.
    if pools.is_empty() {
        return String::new();
    }

    // Get the maximum length of the password.
    let sum_of_min = pools.iter().map(|p| p.minimum).sum::<usize>();
    let length = std::cmp::max(min_length, sum_of_min);

    // Reserve a vector for the password.
    let mut pwd: Vec<char> = Vec::with_capacity(length);

    // Add the minimum number of characters from each pool.
    for pool in pools {
        for _ in 0..pool.minimum {
            let c = pool.char_set.chars().choose(rng).unwrap();
            pwd.push(c);
        }
    }

    // Add the remaining characters.
    let merged_char_set: String = pools.iter().flat_map(|p| p.char_set.chars()).collect();

    for _ in 0..(length - sum_of_min) {
        let c = merged_char_set.chars().choose(rng).unwrap();
        pwd.push(c);
    }

    // Since characters were added in order, shuffle the password.
    pwd.shuffle(rng);

    // Return as a string.
    return pwd.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_pools() {
        let pools = [];
        let pwd = from_pools(&mut rand::thread_rng(), 1000, &pools);
        assert_eq!(pwd.len(), 0);
    }

    #[test]
    fn single_pool() {
        let pools = [Pool {
            name: Some("special"),
            minimum: 3,
            char_set: SPECIAL,
        }];

        let pwd = from_pools(&mut rand::thread_rng(), 0, &pools);
        assert_eq!(pwd.len(), 3);
        assert!(pwd.chars().all(|c| SPECIAL.contains(c)));
    }

    #[test]
    fn minimums_of_zero() {
        let pools = [
            Pool {
                name: Some("alpha"),
                minimum: 1,
                char_set: ALPHA,
            },
            Pool {
                name: Some("numeric"),
                minimum: 0,
                char_set: NUMERIC,
            },
        ];

        let pwd = from_pools(&mut rand::thread_rng(), 10, &pools);
        assert_eq!(pwd.len(), 10);
        assert!(pwd.chars().any(|c| ALPHA.contains(c)));
    }

    #[test]
    fn length_of_two() {
        let pools = [
            Pool {
                name: Some("alpha"),
                minimum: 1,
                char_set: ALPHA,
            },
            Pool {
                name: Some("numeric"),
                minimum: 1,
                char_set: NUMERIC,
            },
        ];

        let pwd = from_pools(&mut rand::thread_rng(), 2, &pools);
        assert_eq!(pwd.len(), 2);
        assert!(pwd.chars().any(|c| ALPHA.contains(c)));
        assert!(pwd.chars().any(|c| NUMERIC.contains(c)));
    }

    #[test]
    fn minimums_exceed_length() {
        let pools = [
            Pool {
                name: Some("alpha"),
                minimum: 2,
                char_set: ALPHA,
            },
            Pool {
                name: Some("numeric"),
                minimum: 2,
                char_set: NUMERIC,
            },
        ];

        let pwd = from_pools(&mut rand::thread_rng(), 0, &pools);
        assert_eq!(pwd.len(), 4);
        assert!(pwd.chars().filter(|c| ALPHA.contains(*c)).count() == 2);
        assert!(pwd.chars().filter(|c| NUMERIC.contains(*c)).count() == 2);
    }

    #[test]
    fn general() {
        let pools = [
            Pool {
                name: Some("alpha"),
                minimum: 3,
                char_set: ALPHA,
            },
            Pool {
                name: Some("numeric"),
                minimum: 3,
                char_set: NUMERIC,
            },
            Pool {
                name: Some("special"),
                minimum: 3,
                char_set: SPECIAL,
            },
        ];

        let pwd = from_pools(&mut rand::thread_rng(), 15, &pools);
        assert_eq!(pwd.len(), 15);
        for pool in pools {
            assert!(pwd.chars().filter(|c| pool.char_set.contains(*c)).count() >= pool.minimum);
        }
    }
}
