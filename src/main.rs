mod filesize;
mod groups;
mod unique_id;
mod walk;
use walk::Walk;

use clap::builder::styling;
use clap::{ArgAction, Parser, ValueEnum, value_parser};
use filesize::FilesizeType;
use humansize::{FormatSizeOptions, format_size};
use std::collections::HashMap;
use std::path::PathBuf;

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

/// Computes disk-usage for the given entries and groups them by extension or file types
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(name = "fss")]
#[command(styles=STYLES)]
struct Cli {
    /// Select how to group the files sizes
    ///
    /// -f   file nam
    /// -e   extension
    /// -t   file type, eg. Images, Videos, Documents...
    #[arg(short, long, default_value = "e", value_parser=["f","e","t"], long_help)]
    group_by: String,

    /// Print the sizes of folders
    #[arg(short, long, default_value_t = false, action=ArgAction::SetTrue)]
    directories: bool,

    /// Output format for file sizes (decimal: MB, binary: MiB)
    #[arg(short, long, default_value = "decimal", value_parser=["decimal", "binary"])]
    size_format: String,

    /// Compute apparent size instead of disk usage
    #[cfg(not(windows))]
    #[arg(short='b', long, default_value_t = false, action=ArgAction::SetTrue)]
    apparent_size: bool,

    /// Set the number of threads to use. Default 3 x num cores
    // Setting the number of threads to 3x the number of cores is a good tradeoff between
    // cold-cache and warm-cache runs. For a cold disk cache, we are limited by disk IO and
    // therefore want the number of threads to be rather large in order for the IO scheduler to
    // plan ahead. On the other hand, the number of threads shouldn't be too high for warm disk
    // caches where we would otherwise pay a higher synchronization overhead.
    #[arg(short = 'j', long, default_value_t =3 * num_cpus::get())]
    threads: usize,

    /// Do not hide filesystem errors
    #[arg(short, long, default_value_t = false, action=ArgAction::SetTrue)]
    verbose: bool,

    /// List of paths
    #[arg(default_value = ".", value_parser=value_parser!(PathBuf))]
    inputs: Vec<PathBuf>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum GroupBy {
    /// Groups by file type. e.g. Images, Videos, Documents...
    Type,

    /// Groups by file extension. This is the default
    #[default]
    Extension,

    /// Groups by file name
    FileName,
}

fn print_result(
    total: u64,
    sizes: HashMap<String, u64>,
    errors: &[walk::Error],
    size_format: &FormatSizeOptions,
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
    sorted_sizes.sort_by_key(|(_k, v)| *v);
    for (group, size) in sorted_sizes.iter().rev() {
        if atty::is(atty::Stream::Stdout) {
            println!("{: >10}\t{}", format_size(*size, size_format), group);
        } else {
            println!("{}\t{}", size, group);
        }
    }
    if atty::is(atty::Stream::Stdout) {
        println!("\nTotal:  {: >10}", format_size(total, size_format));
    } else {
        println!("\nTotal:  {}", total);
    }
}

fn main() {
    let cli = Cli::parse();

    let size_format = match cli.size_format.as_str() {
        "decimal" => humansize::DECIMAL,
        "binary" => humansize::BINARY,
        _ => panic!(
            "Filesize_type should not have been a string different from \"decimal\" or \"binary\""
        ),
    };

    let filesize_type = if cli.apparent_size {
        FilesizeType::ApparentSize
    } else {
        FilesizeType::DiskUsage
    };

    let group_by = match cli.group_by.as_str() {
        "f" => GroupBy::FileName,
        "e" => GroupBy::Extension,
        "t" => GroupBy::Type,
        _ => {
            panic!("--group should not have been a string different from \"t\", \"e\", \"f\",\" ")
        }
    };

    let walk = Walk::new(&cli.inputs, cli.threads, filesize_type, group_by);
    let (total, sizes, errors) = walk.run();
    print_result(total, sizes, &errors, &size_format, cli.verbose);
}
