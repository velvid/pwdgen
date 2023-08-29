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
        long = "alpha",
        default_value = "0",
        help = "Minimum alphabet characters. Will override either upper and loweer."
    )]
    min_alpha: usize,

    #[arg(
        long = "numeric",
        default_value = "0",
        help = "Minimum numeric characters."
    )]
    min_numeric: usize,

    #[arg(
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

    #[arg(short = 'v', long = "verbose", help = "Prints verbose output.")]
    verbose: bool,
}

fn push_to_pool(
    pools: &mut Vec<pwdgen::Pool>,
    name: &'static str,
    minimum: usize,
    chars: &'static str,
) {
    if minimum > 0 {
        pools.push(pwdgen::Pool {
            name,
            minimum,
            chars,
        });
    }
}

fn main() {
    let args = PwdArgs::parse();

    let mut pools: Vec<pwdgen::Pool> = Vec::new();

    if args.min_alpha > 0 {
        push_to_pool(&mut pools, "alpha", args.min_alpha, pwdgen::ALPHA);
    } else {
        push_to_pool(&mut pools, "upper", args.min_upper, pwdgen::UPPER);
        push_to_pool(&mut pools, "lower", args.min_lower, pwdgen::LOWER);
    }
    push_to_pool(&mut pools, "numeric", args.min_numeric, pwdgen::NUMERIC);
    push_to_pool(&mut pools, "special", args.min_special, pwdgen::SPECIAL);

    if pools.is_empty() {
        println!("No character pools specified. Defaulting to alphanumeric characters.");
        push_to_pool(&mut pools, "alpha", 1, pwdgen::ALPHA);
        push_to_pool(&mut pools, "numeric", 1, pwdgen::NUMERIC);
    }

    let start = std::time::Instant::now();

    let pwd = pwdgen::gen_pwd_from_pools(&mut rand::thread_rng(), args.length, &pools);

    let elapsed = start.elapsed();

    cli_clipboard::set_contents(pwd.to_owned()).unwrap();
    println!("{}", "Copied to clipboard!");

    if args.show {
        println!("Password: {}", pwd.clone().cyan());
    }

    if args.verbose {
        // Elapsed time to generate password.
        let mut time = elapsed.as_nanos() as f64;
        let mut units = vec!["s", "ms", "μs", "ns"];

        while !(time < 1000.0) && units.len() > 1 {
            time /= 1000.0;
            units.pop();
        }

        println!("Time to generate: {:.1} {}", time, units.last().unwrap());

        // Percentage of each character pool in final password.
        let mut percentages: Vec<String> = Vec::with_capacity(pools.len());

        let pool_counts = pools
            .iter()
            .map(|p| pwd.chars().filter(|c| p.chars.contains(*c)).count());

        for (pool, count) in pools.iter().zip(pool_counts) {
            let percent = (count as f64 / pwd.len() as f64) * 100.0;
            percentages.push(format!("{}: {:.1}%", pool.name, percent));
        }

        println!("Distribution: {{ {} }}", percentages.join(", "));
    }
}
