use crate::filter::SizeFilter;
use std::path::PathBuf;

use clap::{ArgAction, Parser, ValueEnum, builder::styling, value_parser};
use humansize::format_size;

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
pub struct Cli {
    /// Select how to group the files sizes. [values: e, t, f, d]
    ///
    ///     'e': extension
    ///     't': file type, eg. Images, Videos, Documents...
    ///     'f': file name
    ///     'd': parent directory
    #[arg(short, long, default_value="extension", value_parser=parse_group_by, verbatim_doc_comment)]
    pub group_by: GroupBy,

    /// Limit results based on the size of files using the format <+-><NUM><UNIT>.
    ///    '+': file size must be greater than or equal to this
    ///    '-': file size must be less than or equal to this
    ///
    /// If neither '+' nor '-' is specified, file size must be exactly equal to this.
    ///    'NUM':  The numeric size (e.g. 500)
    ///    'UNIT': The units for NUM. They are not case-sensitive.
    /// Allowed unit values:
    ///     'b':  bytes
    ///     'k':  kilobytes (base ten, 10^3 = 1000 bytes)
    ///     'm':  megabytes
    ///     'g':  gigabytes
    ///     't':  terabytes
    ///     'ki': kibibytes (base two, 2^10 = 1024 bytes)
    ///     'mi': mebibytes
    ///     'gi': gibibytes
    ///     'ti': tebibytes
    #[arg(short = 'S', long, value_parser = SizeFilter::from_string, allow_hyphen_values = true,
        help = "Limit results based on the size of files", verbatim_doc_comment)]
    pub size: Vec<SizeFilter>,

    /// Output format for file sizes (decimal: base-10 MB, binary: base 2 MiB, bytes: raw byte count B)
    #[arg(short, long, default_value_t = FormatOption::Decimal, value_enum)]
    pub size_format: FormatOption,

    /// Compute apparent size instead of disk usage
    #[cfg(not(windows))]
    #[arg(short='b', long, default_value_t = false, action=ArgAction::SetTrue)]
    pub apparent_size: bool,

    /// Set the number of threads to use. Default 3 x num cores
    // Setting the number of threads to 3x the number of cores is a good tradeoff between
    // cold-cache and warm-cache runs. For a cold disk cache, we are limited by disk IO and
    // therefore want the number of threads to be rather large in order for the IO scheduler to
    // plan ahead. On the other hand, the number of threads shouldn't be too high for warm disk
    // caches where we would otherwise pay a higher synchronization overhead.
    #[arg(short = 'j', long, default_value_t =3 * num_cpus::get())]
    pub threads: usize,

    /// Do not hide filesystem errors
    #[arg(short, long, default_value_t = false, action=ArgAction::SetTrue)]
    pub verbose: bool,

    /// List of paths
    #[arg(default_value = ".", value_parser=value_parser!(PathBuf))]
    pub inputs: Vec<PathBuf>,
}

fn parse_group_by(s: &str) -> Result<GroupBy, String> {
    let s = s.to_ascii_lowercase();
    if "extension".starts_with(&s) {
        Ok(GroupBy::Extension)
    } else if "type".starts_with(&s) {
        Ok(GroupBy::Type)
    } else if "filename".starts_with(&s) {
        Ok(GroupBy::FileName)
    } else if "directory".starts_with(&s) {
        Ok(GroupBy::Directory)
    } else {
        Err("Group does is not one of [extension, type, filename, directory]".to_string())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum GroupBy {
    /// Groups by file extension. This is the default
    #[default]
    Extension,

    /// Groups by file type. e.g. Images, Videos, Documents...
    Type,

    /// Groups by file name
    FileName,

    /// Groups by parent directory
    Directory,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum FormatOption {
    Decimal,
    Binary,
    Bytes,
    Auto,
}
impl FormatOption {
    pub fn format(&self, size: u64) -> String {
        match self {
            FormatOption::Decimal => format_size(size, humansize::DECIMAL),
            FormatOption::Binary => format_size(size, humansize::BINARY),
            FormatOption::Bytes => format!("{}", size),
            FormatOption::Auto => {
                if atty::is(atty::Stream::Stdout) {
                    format_size(size, humansize::DECIMAL)
                } else {
                    format!("{}", size)
                }
            }
        }
    }
}
