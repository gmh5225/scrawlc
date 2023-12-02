use colored::Colorize;
use scrawlc;
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

fn print_scanner_error(error: &scrawlc::ScannerError) {
    print!("{} ", "error:".red().bold());

    match error {
        scrawlc::ScannerError::EndOfContent(position) => {
            println!(
                "cannot access {}, end of content",
                position.to_string().purple()
            )
        }
        scrawlc::ScannerError::UnsupportedCharacter(character) => {
            println!(
                "{} is an unsupported character",
                character.to_string().purple()
            )
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut verbose: bool = false;

    for arg in args.iter() {
        if arg == "-v" || arg == "--verbose" {
            verbose = true;
        } else if arg == "-h" || arg == "--help" {
            print_help();

            process::exit(0);
        }
    }

    if args.len() < 3 {
        print_help();

        process::exit(1)
    }

    let input_content = match args.get(1) {
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

    if verbose {
        print!("  {} Scanning ", "[1/1]".bright_white());
    }

    let mut scanner = match scrawlc::Scanner::new(&input_content) {
        Ok(scanner) => scanner,
        Err(error) => {
            if verbose {
                println!("{}", "failed".red());
            }

            print_scanner_error(&error);

            process::exit(1)
        }
    };

    let scan_result = match scanner.scan() {
        Ok(scan_result) => scan_result,
        Err(error) => {
            if verbose {
                println!("{}", "failed".red());
            }

            print_scanner_error(&error);

            process::exit(1)
        }
    };

    if verbose {
        println!("{}", "succeeded".green());
    }

    for (i, token) in scan_result.iter().enumerate() {
        println!(
            "    {}{} {}",
            i.to_string().bright_green(),
            ":".bright_green(),
            token
        );
    }
}
