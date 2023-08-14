use rand::{seq::IteratorRandom, seq::SliceRandom};
use std::cmp;
use weighted_rand::builder::*;

pub const ALPHA: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const NUMERIC: &'static str = "0123456789";
pub const SPECIAL: &'static str = "!@#$%^&*()_+-=[]{};':\",./<>?\\|";

pub fn gen_pwd(
    rng: &mut rand::rngs::ThreadRng,
    min_length: usize,
    min_alpha: usize,
    min_numeric: usize,
    min_special: usize,
) -> String {
    // Get the maximum length of the password.
    let min_sum = min_alpha + min_numeric + min_special;
    let length = cmp::max(min_length, min_sum);

    // Populate password from character sets.
    let mut pwd: Vec<char> = Vec::new();
    pwd.reserve(length);

    let sets = [ALPHA, NUMERIC, SPECIAL];
    let mins = [min_alpha, min_numeric, min_special];
    let weights = sets.map(|s| s.len() as u32);
    let table = WalkerTableBuilder::new(&weights).build();

    for (set, min) in sets.iter().zip(mins.iter()) {
        for _ in 0..*min {
            let c = set.chars().choose(rng).unwrap();
            pwd.push(c);
        }
    }

    let remainder = length - min_sum;
    for _ in 0..remainder {
        let i = table.next_rng(rng);
        let c = sets[i].chars().choose(rng).unwrap();
        pwd.push(c);
    }

    pwd.shuffle(rng);

    // Return password as a string.
    pwd.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rng() -> rand::rngs::ThreadRng {
        // Can change this to a different RNG if needed.
        rand::thread_rng()
    }

    #[test]
    fn normal_case() {
        let pwd = gen_pwd(&mut test_rng(), 32, 8, 8, 8);
        let alpha_count = pwd.chars().filter(|c| ALPHA.contains(*c)).count();
        let numeric_count = pwd.chars().filter(|c| NUMERIC.contains(*c)).count();
        let special_count = pwd.chars().filter(|c| SPECIAL.contains(*c)).count();

        assert!(alpha_count >= 8);
        assert!(numeric_count >= 8);
        assert!(special_count >= 8);
        assert_eq!(pwd.len(), 32);
    }

    #[test]
    fn larger_min_total() {
        let pwd = gen_pwd(&mut test_rng(), 16, 8, 8, 8);
        let alpha_count = pwd.chars().filter(|c| ALPHA.contains(*c)).count();
        let numeric_count = pwd.chars().filter(|c| NUMERIC.contains(*c)).count();
        let special_count = pwd.chars().filter(|c| SPECIAL.contains(*c)).count();

        assert_eq!(alpha_count, 8);
        assert_eq!(numeric_count, 8);
        assert_eq!(special_count, 8);
        assert_eq!(pwd.len(), 24);
    }

    #[test]
    fn all_alpha() {
        let pwd = gen_pwd(&mut test_rng(), 16, 16, 0, 0);
        let count = pwd.chars().filter(|c| ALPHA.contains(*c)).count();

        assert!(count == 16);
        assert!(pwd.len() == 16);
    }

    #[test]
    fn all_numeric() {
        let pwd = gen_pwd(&mut test_rng(), 16, 0, 16, 0);
        let count = pwd.chars().filter(|c| NUMERIC.contains(*c)).count();

        assert!(count == 16);
        assert!(pwd.len() == 16);
    }

    #[test]
    fn all_special() {
        let pwd = gen_pwd(&mut test_rng(), 16, 0, 0, 16);
        let count = pwd.chars().filter(|c| SPECIAL.contains(*c)).count();

        assert!(count == 16);
        assert!(pwd.len() == 16);
    }

    #[test]
    fn zero_min() {
        let pwd = gen_pwd(&mut test_rng(), 16, 0, 0, 0);

        assert!(pwd.len() == 16);
    }
}
