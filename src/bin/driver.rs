use colored::Colorize;
use std::{env, process};

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut files: Vec<String> = Vec::new();

    if args.len() == 1 {
        println!("{} ({})", "scrawlc".green(), PKG_VERSION.bold());

        println!(
            "{} scrawlc {}",
            "usage:".yellow().bold(),
            "[file...]".purple()
        );

        process::exit(1)
    } else {
        for arg in args.iter() {
            files.push(arg.clone());
        }
    }
}
