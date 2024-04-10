mod pwdgen;
use pwdgen::chars::*;

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
        short = 'a',
        long = "alpha",
        help = "Minimum alphabet characters. If flag is not set, won't be added to character pool."
    )]
    min_alpha: Option<Option<usize>>,

    #[arg(
        short = 'n',
        long = "numeric",
        help = "Minimum numeric characters. If flag is not set, won't be added to character pool."
    )]
    min_numeric: Option<Option<usize>>,

    #[arg(
        short = 's',
        long = "special",
        help = "Minimum special characters. If flag is not set, won't be added to character pool."
    )]
    min_special: Option<Option<usize>>,

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

    #[arg(
        short = 'v',
        long = "verbose",
        help = "Prints information such as time taken to generate password and character pool distribution."
    )]
    verbose: bool,
}

#[inline]
fn add_to_pool(
    pool: &mut Vec<Set>,
    name: &'static str,
    min: Option<Option<usize>>,
    char_set: &'static str,
) {
    if min.is_some() {
        pool.push(Set {
            name,
            minimum: min.unwrap().unwrap_or(0),
            chars: char_set,
        });
    }
}

fn main() {
    let args = PwdArgs::parse();

    let mut pool = Vec::new();
    add_to_pool(&mut pool, "alpha", args.min_alpha, ALPHA);
    add_to_pool(&mut pool, "numeric", args.min_numeric, NUMERIC);
    add_to_pool(&mut pool, "special", args.min_special, SPECIAL);

    if pool.is_empty() {
        println!("No character sets specified to add to pool. Defaulting to alphanumeric.");
        add_to_pool(&mut pool, "alpha", Some(Some(1)), ALPHA);
        add_to_pool(&mut pool, "numeric", Some(Some(1)), NUMERIC);
    }

    let start = std::time::Instant::now();
    let pwd = pwdgen::from_pool(&mut rand::thread_rng(), args.length, &pool).unwrap();
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
        let mut percentages = Vec::with_capacity(pool.len());

        let pool_counts = pool
            .iter()
            .map(|p| pwd.chars().filter(|c| p.chars.contains(*c)).count());

        for (pool, count) in pool.iter().zip(pool_counts) {
            let percent = (count as f64 / pwd.len() as f64) * 100.0;
            percentages.push(format!(
                "{}: {}",
                pool.name.magenta(),
                format!("{:.2}%", percent).bright_magenta()
            ));
        }

        println!("Distribution: {{ {} }}", percentages.join(", "));
    }
}
