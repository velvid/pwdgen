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
}

fn main() {
    let args = PwdArgs::parse();

    let pwd = gen_pwd(
        &mut rand::thread_rng(),
        args.length,
        args.min_alpha,
        args.min_numeric,
        args.min_special,
    );

    println!("{}", pwd);
}
