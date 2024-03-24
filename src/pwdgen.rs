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
    fn general() {
        let mut rng = rand::thread_rng();

        let pools = [
            Pool {
                name: "alpha",
                minimum: 2,
                chars: ALPHA,
            },
            Pool {
                name: "numeric",
                minimum: 2,
                chars: NUMERIC,
            },
            Pool {
                name: "special",
                minimum: 2,
                chars: SPECIAL,
            },
        ];

        let pwd = gen_pwd_from_pools(&mut rng, 8, &pools);

        assert_eq!(pwd.len(), 8);
        for pool in &pools {
            assert!(pwd.chars().filter(|c| pool.chars.contains(*c)).count() >= pool.minimum);
        }
    }
}
