use colored::Colorize;
use std::{env, fs, path, process};

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    println!("{} ({})", "scrawlc".green(), PKG_VERSION.bold());

    println!(
        "{} scrawlc {} {}",
        "usage:".yellow().bold(),
        "<input file>".purple(),
        "<output file>".purple()
    );
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        print_help();

        process::exit(1)
    }

    let _input_content = match args.get(1) {
        Some(file_path) => match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(error) => {
                if !path::PathBuf::from(file_path).is_file() {
                    println!(
                        "{} {} is not a file",
                        "error:".red().bold(),
                        file_path.purple()
                    );

                    process::exit(1)
                } else {
                    println!(
                        "{} could not read {}; {}",
                        "error:".red().bold(),
                        file_path.purple(),
                        error.to_string().bright_black().italic()
                    );

                    process::exit(1)
                }
            }
        },
        None => {
            print_help();

            process::exit(1)
        }
    };

    let output_file_path = match args.get(2) {
        Some(arg) => path::PathBuf::from(arg),
        None => {
            print_help();

            process::exit(1)
        }
    };

    if !output_file_path.is_file() {
        println!(
            "{} {} is not a file",
            "error:".red().bold(),
            output_file_path
                .to_str()
                .unwrap_or(&output_file_path.to_string_lossy())
                .purple()
        );

        process::exit(1)
    }
}
