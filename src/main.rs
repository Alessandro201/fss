pub mod cli;
mod filesize;
mod filter;
mod groups;
mod unique_id;
mod walk;
use clap::Parser;
use colored::Colorize;
use filter::SizeFilter;
use walk::Walk;

use filesize::FilesizeType;
use humansize::{FormatSizeOptions, format_size};
use std::collections::HashMap;

fn print_result(
    total: u64,
    sizes: HashMap<String, u64>,
    errors: &[walk::Error],
    size_format: Option<FormatSizeOptions>,
    size_filter: Vec<SizeFilter>,
    verbose: bool,
) {
    if verbose {
        for err in errors {
            match err {
                walk::Error::NoMetadataForPath(path) => {
                    eprintln!(
                        "fss: could not retrieve metadata for path '{}'",
                        path.to_string_lossy()
                    );
                }
                walk::Error::CouldNotReadDir(path) => {
                    eprintln!(
                        "fss: could not read contents of directory '{}'",
                        path.to_string_lossy()
                    );
                }
            }
        }
    } else if !errors.is_empty() {
        eprintln!(
            "[fss warning] the results may be tainted. Re-run with -v/--verbose to print all errors."
        );
    }

    let mut sorted_sizes: Vec<(String, u64)> = sizes.into_iter().collect();
    sorted_sizes.sort_unstable_by_key(|(_k, v)| *v);
    for (group, size) in sorted_sizes {
        if size_filter.iter().any(|f| !f.is_within(size)) {
            continue;
        }

        if let Some(size_format) = size_format {
            println!("{: >10}\t{}", format_size(size, size_format), group);
        } else {
            println!("{}\t{}", size, group);
        }
    }

    if atty::is(atty::Stream::Stdout) {
        if let Some(size_format) = size_format {
            println!(
                "\n{}  \n{: >10}",
                "Total: ".bold().cyan(),
                format_size(total, size_format)
            );
        } else {
            println!("\n{}  \n{: >10}", "Total: ".bold().cyan(), total);
        }
    }
}

fn main() {
    let cli = cli::Cli::parse();

    let size_format = match cli.size_format.as_str() {
        "decimal" => Some(humansize::DECIMAL),
        "binary" => Some(humansize::BINARY),
        "bytes" => None,
        _ => panic!(
            "Filesize_type should not have been a string different from \"decimal\" or \"binary\""
        ),
    };

    let filesize_type = if cli.apparent_size {
        FilesizeType::ApparentSize
    } else {
        FilesizeType::DiskUsage
    };

    let walk = Walk::new(&cli.inputs, cli.threads, filesize_type, cli.group_by);
    let (total, sizes, errors) = walk.run();
    print_result(total, sizes, &errors, size_format, cli.size, cli.verbose);
}
