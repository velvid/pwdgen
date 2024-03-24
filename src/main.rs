mod pwdgen;

use clap::Parser;
use cli_clipboard;
use colored::*;

#[derive(Parser)]
struct PwdArgs {
    #[arg(
        short = 'l',
        long = "length",
        default_value = "16",
        help = "Length of the password. Will be overriden if less than the sum of minimum characters."
    )]
    length: usize,

    #[arg(
        long = "upper",
        default_value = "0",
        help = "Minimum uppercase characters."
    )]
    min_upper: usize,

    #[arg(
        long = "lower",
        default_value = "0",
        help = "Minimum lowercase characters."
    )]
    min_lower: usize,

    #[arg(
        short = 'a',
        long = "alpha",
        default_value = "0",
        help = "Minimum alphabet characters. Will override either upper or lower."
    )]
    min_alpha: usize,

    #[arg(
        short = 'n',
        long = "numeric",
        default_value = "0",
        help = "Minimum numeric characters."
    )]
    min_numeric: usize,

    #[arg(
        short = 's',
        long = "special",
        default_value = "0",
        help = "Minimum special characters."
    )]
    min_special: usize,

    #[arg(
        long = "show",
        default_value = "false",
        help = "Prints the password after generating."
    )]
    show: bool,

    #[arg(
        long = "copy",
        default_value = "false",
        help = "Copies the generated password to clipboard."
    )]
    copy: bool,

    #[arg(short = 'v', long = "verbose", help = "Prints verbose output.")]
    verbose: bool,
}

#[inline]
fn emplace(
    pools: &mut Vec<pwdgen::Pool>,
    name: Option<&'static str>,
    minimum: usize,
    char_set: &'static str,
) {
    if minimum > 0 {
        pools.push(pwdgen::Pool {
            name,
            minimum,
            char_set,
        });
    }
}

fn main() {
    let args = PwdArgs::parse();

    let mut pools = Vec::new();

    if args.min_alpha > 0 {
        emplace(&mut pools, Some("alpha"), args.min_alpha, pwdgen::ALPHA);
    } else {
        emplace(&mut pools, Some("upper"), args.min_upper, pwdgen::UPPER);
        emplace(&mut pools, Some("lower"), args.min_lower, pwdgen::LOWER);
    }
    emplace(&mut pools, Some("numeric"), args.min_numeric, pwdgen::NUMERIC);
    emplace(&mut pools, Some("special"), args.min_special, pwdgen::SPECIAL);

    if pools.is_empty() {
        println!("No character pools specified. Defaulting to alphanumeric characters.");
        emplace(&mut pools, Some("alpha"), 1, pwdgen::ALPHA);
        emplace(&mut pools, Some("numeric"), 1, pwdgen::NUMERIC);
    }

    let start = std::time::Instant::now();
    let pwd = pwdgen::from_pools(&mut rand::thread_rng(), args.length, &pools);
    let elapsed = start.elapsed();

    if args.copy {
        match cli_clipboard::set_contents(pwd.to_owned()) {
            Ok(_) => println!("{}", "Copied to clipboard!".green()),
            _ => println!("{}", "Failed to copy to clipboard".red()),
        }
    }

    if args.show {
        println!("Password: {}", pwd.cyan());
    }

    if args.verbose {
        // Elapsed time to generate password.
        let mut time = elapsed.as_nanos() as f64;
        let mut units = vec!["s", "ms", "μs", "ns"];

        while time >= 1000.0 && units.len() > 1 {
            time /= 1000.0;
            units.pop();
        }

        let str_time = format!("{:.2} {}", time, units.last().unwrap());
        println!("Time to generate: {}", str_time.yellow());

        // Percentage of each character pool in final password.
        let mut percentages = Vec::with_capacity(pools.len());

        let pool_counts = pools
            .iter()
            .map(|p| pwd.chars().filter(|c| p.char_set.contains(*c)).count());

        for (pool, count) in pools.iter().zip(pool_counts) {
            let percent = (count as f64 / pwd.len() as f64) * 100.0;
            percentages.push(format!(
                "{}: {}",
                pool.name.unwrap_or("???").magenta(),
                format!("{:.2}%", percent).bright_magenta()
            ));
        }

        println!("Distribution: {{ {} }}", percentages.join(", "));
    }
}
