mod pwdgen;
use crate::pwdgen::gen_pwd;

use clap::Parser;

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
        default_value = "1",
        help = "Minimum alphabet characters."
    )]
    min_alpha: usize,

    #[arg(
        short = 'n',
        long = "numeric",
        default_value = "1",
        help = "Minimum numeric characters."
    )]
    min_numeric: usize,

    #[arg(
        short = 's',
        long = "special",
        default_value = "1",
        help = "Minimum special characters."
    )]
    min_special: usize,

    #[arg(short = 'v', long = "verbose", help = "Prints verbose output.")]
    verbose: bool,
}

fn main() {
    let args = PwdArgs::parse();

    let start = std::time::Instant::now();

    let pwd = gen_pwd(
        &mut rand::thread_rng(),
        args.length,
        args.min_alpha,
        args.min_numeric,
        args.min_special,
    );

    let elapsed = start.elapsed();

    println!("{}", pwd,);

    if args.verbose {
        // Print elapsed time to generate password.
        print!("Took: ");
        match elapsed.as_nanos() {
            0..=999 => println!("{} ns", elapsed.as_nanos()),
            1_000..=999_999 => println!("{} us", elapsed.as_micros()),
            1_000_000..=999_999_999 => println!("{} ms", elapsed.as_millis()),
            _ => println!("{} s", elapsed.as_secs()),
        }

        // Print percentage of each character type.
        let alpha_count = pwd.chars().filter(|c| pwdgen::ALPHA.contains(*c)).count();
        let numeric_count = pwd.chars().filter(|c| pwdgen::NUMERIC.contains(*c)).count();
        let special_count = pwd.chars().filter(|c| pwdgen::SPECIAL.contains(*c)).count();
        println!(
            "Alpha: {:.2}%, Numeric: {:.2}%, Special: {:.2}%",
            alpha_count as f64 / pwd.len() as f64 * 100.0,
            numeric_count as f64 / pwd.len() as f64 * 100.0,
            special_count as f64 / pwd.len() as f64 * 100.0,
        );
    }
}
