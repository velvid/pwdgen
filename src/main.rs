mod pwdgen;

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

    let pools = [
        pwdgen::Pool {
            name: "alpha",
            minimum: args.min_alpha,
            chars: pwdgen::ALPHA,
        },
        pwdgen::Pool {
            name: "numeric",
            minimum: args.min_numeric,
            chars: pwdgen::NUMERIC,
        },
        pwdgen::Pool {
            name: "special",
            minimum: args.min_special,
            chars: pwdgen::SPECIAL,
        },
    ];

    let start = std::time::Instant::now();

    let pwd = pwdgen::gen_pwd_from_pools(&mut rand::thread_rng(), args.length, &pools);

    let elapsed = start.elapsed();

    println!("{}", pwd);

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
