use rand::seq::{IteratorRandom, SliceRandom};

#[allow(dead_code)]
pub mod chars {
    pub const UPPER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    pub const LOWER: &'static str = "abcdefghijklmnopqrstuvwxyz";
    pub const ALPHA: &'static str = constcat::concat!(UPPER, LOWER);
    pub const NUMERIC: &'static str = "0123456789";
    pub const SPECIAL: &'static str = "!@#$%^&*()_+-=[]{};':\",./<>?\\|";
    pub const ALL: &'static str = constcat::concat!(ALPHA, NUMERIC, SPECIAL);

    pub struct Set {
        pub name: &'static str,
        pub minimum: usize,
        pub chars: &'static str,
    }
}

#[allow(dead_code)]
pub fn from_pool<R>(
    rng: &mut R,
    min_length: usize,
    pools: &[chars::Set],
) -> Result<String, &'static str>
where
    R: rand::Rng + ?Sized,
{
    if pools.is_empty() {
        return Err("No character sets specified.");
    }

    for pool in pools {
        if pool.chars.is_empty() {
            return Err("Empty character set in pool.");
        }
    }

    let sum_of_min = pools.iter().map(|p| p.minimum).sum::<usize>();
    let length = std::cmp::max(min_length, sum_of_min);

    let mut pwd: Vec<char> = Vec::with_capacity(length);

    // Add the minimum number of characters from each pool.
    for pool in pools {
        for _ in 0..pool.minimum {
            let c = pool.chars.chars().choose(rng).unwrap();
            pwd.push(c);
        }
    }

    // Add the remaining characters.
    let merged_char_set: String = pools.iter().flat_map(|p| p.chars.chars()).collect();

    for _ in 0..(length - sum_of_min) {
        let c = merged_char_set.chars().choose(rng).unwrap();
        pwd.push(c);
    }

    pwd.shuffle(rng);

    return Ok(pwd.into_iter().collect());
}

#[allow(dead_code)]
pub fn from_params<R>(
    rng: &mut R,
    min_length: usize,
    min_alpha: Option<usize>,
    min_numeric: Option<usize>,
    min_special: Option<usize>,
) -> Result<String, &'static str>
where
    R: rand::Rng + ?Sized,
{
    use chars::*;

    let sum_of_min = min_alpha.unwrap_or(0) + min_numeric.unwrap_or(0) + min_special.unwrap_or(0);

    if sum_of_min == 0 {
        return Err("No character sets specified.");
    }

    let length = std::cmp::max(min_length, sum_of_min);
    let mut pwd: Vec<char> = Vec::with_capacity(length);

    for _ in 0..min_alpha.unwrap_or(0) {
        let c = ALPHA.chars().choose(rng).unwrap();
        pwd.push(c);
    }

    for _ in 0..min_numeric.unwrap_or(0) {
        let c = NUMERIC.chars().choose(rng).unwrap();
        pwd.push(c);
    }

    for _ in 0..min_special.unwrap_or(0) {
        let c = SPECIAL.chars().choose(rng).unwrap();
        pwd.push(c);
    }

    let mut merged_chars = String::with_capacity(chars::ALL.len());

    if min_alpha.is_some() {
        merged_chars.push_str(ALPHA);
    }

    if min_numeric.is_some() {
        merged_chars.push_str(NUMERIC);
    }

    if min_special.is_some() {
        merged_chars.push_str(SPECIAL);
    }

    for _ in 0..(length - sum_of_min) {
        let c = merged_chars.chars().choose(rng).unwrap();
        pwd.push(c);
    }

    pwd.shuffle(rng);

    return Ok(pwd.into_iter().collect());
}
