use rand::{seq::IteratorRandom, seq::SliceRandom, Rng};
use std::cmp;

#[cfg(test)]
use rand::SeedableRng;

struct CHARS;
impl CHARS {
    const ALPHA: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMERIC: &'static str = "0123456789";
    const SPECIAL: &'static str = "!@#$%^&*()_+-=[]{};':\",./<>?\\|";
}

pub fn gen_pwd(
    rng: &mut dyn rand::RngCore,
    length: usize,
    min_alpha: usize,
    min_numeric: usize,
    min_special: usize,
) -> String {
    // Get the maximum length of the password.
    let min_sum = min_alpha + min_numeric + min_special;
    let length = cmp::max(length, min_sum);

    // Get random counts for each character set.
    let max_alpha = length - min_numeric - min_special;
    let alpha_count = rng.gen_range(min_alpha..=max_alpha);

    let max_numeric = length - alpha_count - min_special;
    let numeric_count = rng.gen_range(min_numeric..=max_numeric);

    let special_count = length - alpha_count - numeric_count;

    // Populate password from character sets.
    let mut pwd: Vec<char> = Vec::new();
    pwd.reserve(length);

    for _ in 0..alpha_count {
        pwd.push(CHARS::ALPHA.chars().choose(rng).unwrap());
    }
    for _ in 0..numeric_count {
        pwd.push(CHARS::NUMERIC.chars().choose(rng).unwrap());
    }
    for _ in 0..special_count {
        pwd.push(CHARS::SPECIAL.chars().choose(rng).unwrap());
    }

    pwd.shuffle(rng);

    // Return password as a string.
    pwd.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seeded_rng() -> rand::rngs::StdRng {
        rand::rngs::StdRng::from_seed([0; 32])
    }

    #[test]
    fn normal_case() {
        let pwd = gen_pwd(&mut seeded_rng(), 32, 8, 8, 8);
        let alpha_count = pwd.chars().filter(|c| CHARS::ALPHA.contains(*c)).count();
        let numeric_count = pwd.chars().filter(|c| CHARS::NUMERIC.contains(*c)).count();
        let special_count = pwd.chars().filter(|c| CHARS::SPECIAL.contains(*c)).count();

        assert!(alpha_count >= 8);
        assert!(numeric_count >= 8);
        assert!(special_count >= 8);
        assert_eq!(pwd.len(), 32);
    }

    #[test]
    fn larger_min_total() {
        let pwd = gen_pwd(&mut seeded_rng(), 16, 8, 8, 8);
        let alpha_count = pwd.chars().filter(|c| CHARS::ALPHA.contains(*c)).count();
        let numeric_count = pwd.chars().filter(|c| CHARS::NUMERIC.contains(*c)).count();
        let special_count = pwd.chars().filter(|c| CHARS::SPECIAL.contains(*c)).count();

        assert_eq!(alpha_count, 8);
        assert_eq!(numeric_count, 8);
        assert_eq!(special_count, 8);
        assert_eq!(pwd.len(), 24);
    }

    #[test]
    fn all_alpha() {
        let pwd = gen_pwd(&mut seeded_rng(), 16, 16, 0, 0);
        let count = pwd.chars().filter(|c| CHARS::ALPHA.contains(*c)).count();

        assert!(count == 16);
        assert!(pwd.len() == 16);
    }

    #[test]
    fn all_numeric() {
        let pwd = gen_pwd(&mut seeded_rng(), 16, 0, 16, 0);
        let count = pwd.chars().filter(|c| CHARS::NUMERIC.contains(*c)).count();

        assert!(count == 16);
        assert!(pwd.len() == 16);
    }

    #[test]
    fn all_special() {
        let pwd = gen_pwd(&mut seeded_rng(), 16, 0, 0, 16);
        let count = pwd.chars().filter(|c| CHARS::SPECIAL.contains(*c)).count();

        assert!(count == 16);
        assert!(pwd.len() == 16);
    }

    #[test]
    fn zero_min() {
        let pwd = gen_pwd(&mut seeded_rng(), 16, 0, 0, 0);

        assert!(pwd.len() == 16);
    }
}
