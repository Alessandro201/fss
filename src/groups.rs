use std::{fmt::Display, path::Path};

use clap::ValueEnum;
use fnv::FnvHashMap;
use lazy_static::lazy_static;

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

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileType {
    Image,
    Video,
    Document,
    Executable,
    Archive,
    Audio,
    Code,
    GenomicData,
    Other,
}

lazy_static! {
    static ref FILETYPE_MAP: FnvHashMap<&'static str, FileType> = {
        let mut hm = FnvHashMap::default();

        hm.insert("jpg", FileType::Image);
        hm.insert("jpeg", FileType::Image);
        hm.insert("jpegxl", FileType::Image);
        hm.insert("png", FileType::Image);
        hm.insert("tiff", FileType::Image);
        hm.insert("raw", FileType::Image);
        hm.insert("nef", FileType::Image);
        hm.insert("webp", FileType::Image);
        hm.insert("psd", FileType::Image);
        hm.insert("heic", FileType::Image);
        hm.insert("gif", FileType::Image);
        hm.insert("avif", FileType::Image);
        hm.insert("dng", FileType::Image);
        hm.insert("svg", FileType::Image);
        hm.insert("bmp", FileType::Image);

        hm.insert("mp4", FileType::Video);
        hm.insert("mkv", FileType::Video);
        hm.insert("avi", FileType::Video);
        hm.insert("webm", FileType::Video);
        hm.insert("flv", FileType::Video);
        hm.insert("f4v", FileType::Video);
        hm.insert("gifv", FileType::Video);
        hm.insert("mpeg", FileType::Video);
        hm.insert("mpg", FileType::Video);
        hm.insert("mov", FileType::Video);
        hm.insert("wmv", FileType::Video);
        hm.insert("3gp", FileType::Video);
        hm.insert("aaf", FileType::Video);
        hm.insert("avchd", FileType::Video);

        hm.insert("pdf", FileType::Document);
        hm.insert("txt", FileType::Document);
        hm.insert("docx", FileType::Document);
        hm.insert("doc", FileType::Document);
        hm.insert("xlsx", FileType::Document);
        hm.insert("xls", FileType::Document);
        hm.insert("csv", FileType::Document);
        hm.insert("tsv", FileType::Document);
        hm.insert("md", FileType::Document);
        hm.insert("odt", FileType::Document);
        hm.insert("fodt", FileType::Document);
        hm.insert("pages", FileType::Document);
        hm.insert("rtf", FileType::Document);
        hm.insert("tex", FileType::Document);
        hm.insert("latex", FileType::Document);
        hm.insert("epub", FileType::Document);
        hm.insert("kpub", FileType::Document);
        hm.insert("ppt", FileType::Document);
        hm.insert("pptx", FileType::Document);
        hm.insert("otp", FileType::Document);
        hm.insert("odp", FileType::Document);
        hm.insert("pot", FileType::Document);
        hm.insert("pps", FileType::Document);
        hm.insert("bib", FileType::Document);
        hm.insert("log", FileType::Document);
        hm.insert("tmp", FileType::Document);
        hm.insert("temp", FileType::Document);

        hm.insert("py", FileType::Code);
        hm.insert("pyc", FileType::Code);
        hm.insert("pyo", FileType::Code);
        hm.insert("xml", FileType::Code);
        hm.insert("html", FileType::Code);
        hm.insert("htm", FileType::Code);
        hm.insert("htmx", FileType::Code);
        hm.insert("xhtml", FileType::Code);
        hm.insert("xht", FileType::Code);
        hm.insert("css", FileType::Code);
        hm.insert("js", FileType::Code);
        hm.insert("jsx", FileType::Code);
        hm.insert("json", FileType::Code);
        hm.insert("yaml", FileType::Code);
        hm.insert("toml", FileType::Code);
        hm.insert("ts", FileType::Code);
        hm.insert("c", FileType::Code);
        hm.insert("cpp", FileType::Code);
        hm.insert("h", FileType::Code);
        hm.insert("rs", FileType::Code);
        hm.insert("r", FileType::Code);
        hm.insert("go", FileType::Code);
        hm.insert("zig", FileType::Code);
        hm.insert("awk", FileType::Code);
        hm.insert("cs", FileType::Code);
        hm.insert("csproj", FileType::Code);
        hm.insert("ici", FileType::Code);
        hm.insert("ipynb", FileType::Code);
        hm.insert("kt", FileType::Code);
        hm.insert("lua", FileType::Code);
        hm.insert("php", FileType::Code);
        hm.insert("pl", FileType::Code);
        hm.insert("pm", FileType::Code);
        hm.insert("ps1", FileType::Code);
        hm.insert("sh", FileType::Code);
        hm.insert("fish", FileType::Code);
        hm.insert("asm", FileType::Code);
        hm.insert("d", FileType::Code);
        hm.insert("vim", FileType::Code);
        hm.insert("java", FileType::Code);
        hm.insert("lisp", FileType::Code);
        hm.insert("php3", FileType::Code);
        hm.insert("php4", FileType::Code);
        hm.insert("php5", FileType::Code);
        hm.insert("phps", FileType::Code);
        hm.insert("vb", FileType::Code);
        hm.insert("sql", FileType::Code);

        hm.insert("exe", FileType::Executable);
        hm.insert("apk", FileType::Executable);
        hm.insert("o", FileType::Executable);
        hm.insert("so", FileType::Executable);
        hm.insert("app", FileType::Executable);
        hm.insert("dll", FileType::Executable);
        hm.insert("elf", FileType::Executable);
        hm.insert("jar", FileType::Executable);
        hm.insert("lib", FileType::Executable);

        hm.insert("mp3", FileType::Audio);
        hm.insert("aiff", FileType::Audio);
        hm.insert("aif", FileType::Audio);
        hm.insert("aifc", FileType::Audio);
        hm.insert("wav", FileType::Audio);
        hm.insert("flac", FileType::Audio);
        hm.insert("wma", FileType::Audio);
        hm.insert("dts", FileType::Audio);
        hm.insert("ac3", FileType::Audio);
        hm.insert("aac", FileType::Audio);
        hm.insert("ots", FileType::Audio);
        hm.insert("ogg", FileType::Audio);

        hm.insert("gz", FileType::Archive);
        hm.insert("gzip", FileType::Archive);
        hm.insert("zst", FileType::Archive);
        hm.insert("zstd", FileType::Archive);
        hm.insert("zip", FileType::Archive);
        hm.insert("7z", FileType::Archive);
        hm.insert("7zip", FileType::Archive);
        hm.insert("rar", FileType::Archive);
        hm.insert("tar", FileType::Archive);
        hm.insert("bin", FileType::Archive);
        hm.insert("dat", FileType::Archive);
        hm.insert("bz2", FileType::Archive);
        hm.insert("pak", FileType::Archive);
        hm.insert("par", FileType::Archive);
        hm.insert("pax", FileType::Archive);
        hm.insert("sqlite", FileType::Archive);
        hm.insert("sq", FileType::Archive);
        hm.insert("vbox", FileType::Archive);

        hm.insert("bam", FileType::GenomicData);
        hm.insert("bai", FileType::GenomicData);
        hm.insert("sam", FileType::GenomicData);
        hm.insert("bed", FileType::GenomicData);
        hm.insert("gtf", FileType::GenomicData);
        hm.insert("gtf2", FileType::GenomicData);
        hm.insert("gtf3", FileType::GenomicData);
        hm.insert("gff", FileType::GenomicData);
        hm.insert("gff2", FileType::GenomicData);
        hm.insert("gff3", FileType::GenomicData);
        hm.insert("bedpe", FileType::GenomicData);
        hm.insert("cram", FileType::GenomicData);
        hm.insert("sra", FileType::GenomicData);
        hm.insert("fastq", FileType::GenomicData);
        hm.insert("fasta", FileType::GenomicData);
        hm.insert("fa", FileType::GenomicData);
        hm.insert("fq", FileType::GenomicData);
        hm.insert("fasterq", FileType::GenomicData);
        hm.insert("embl", FileType::GenomicData);
        hm.insert("genbank", FileType::GenomicData);
        hm.insert("pdb", FileType::GenomicData);
        hm.insert("ncbi", FileType::GenomicData);
        hm.insert("maf", FileType::GenomicData);
        hm.insert("nwk", FileType::GenomicData);
        hm.insert("phd", FileType::GenomicData);
        hm.insert("vcf", FileType::GenomicData);
        hm.insert("pod5", FileType::GenomicData);

        hm
    };
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FileType {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let ext = path
            .as_ref()
            .extension()
            .unwrap_or(path.as_ref().file_name().unwrap_or_default())
            .to_ascii_lowercase()
            .to_str()
            .unwrap_or_default()
            .to_owned();
        FileType::get_filetype(&ext)
    }

    #[inline(always)]
    pub fn get_filetype<S: AsRef<str>>(ext: &S) -> Self {
        FILETYPE_MAP
            .get(ext.as_ref())
            .map_or(FileType::Other, |ft| ft.to_owned())
    }

    // #[inline(always)]
    // pub fn get_filetype<S: AsRef<str>>(ext: &S) -> Self {
    //     match ext.as_ref().to_lowercase().as_str() {
    //         "jpg" => Self::Image,
    //         "jpeg" => Self::Image,
    //         "jpegxl" => Self::Image,
    //         "png" => Self::Image,
    //         "tiff" => Self::Image,
    //         "raw" => Self::Image,
    //         "nef" => Self::Image,
    //         "webp" => Self::Image,
    //         "psd" => Self::Image,
    //         "heic" => Self::Image,
    //         "gif" => Self::Image,
    //         "avif" => Self::Image,
    //         "dng" => Self::Image,
    //         "svg" => Self::Image,
    //         "bmp" => Self::Image,
    //
    //         "mp4" => Self::Video,
    //         "mkv" => Self::Video,
    //         "avi" => Self::Video,
    //         "webm" => Self::Video,
    //         "flv" => Self::Video,
    //         "f4v" => Self::Video,
    //         "gifv" => Self::Video,
    //         "mpeg" => Self::Video,
    //         "mpg" => Self::Video,
    //         "mov" => Self::Video,
    //         "wmv" => Self::Video,
    //         "3gp" => Self::Video,
    //         "aaf" => Self::Video,
    //         "avchd" => Self::Video,
    //
    //         "pdf" => Self::Document,
    //         "txt" => Self::Document,
    //         "docx" => Self::Document,
    //         "doc" => Self::Document,
    //         "xlsx" => Self::Document,
    //         "xls" => Self::Document,
    //         "csv" => Self::Document,
    //         "tsv" => Self::Document,
    //         "md" => Self::Document,
    //         "odt" => Self::Document,
    //         "fodt" => Self::Document,
    //         "pages" => Self::Document,
    //         "rtf" => Self::Document,
    //         "tex" => Self::Document,
    //         "latex" => Self::Document,
    //         "epub" => Self::Document,
    //         "kpub" => Self::Document,
    //         "ppt" => Self::Document,
    //         "pptx" => Self::Document,
    //         "otp" => Self::Document,
    //         "odp" => Self::Document,
    //         "pot" => Self::Document,
    //         "pps" => Self::Document,
    //         "bib" => Self::Document,
    //         "log" => Self::Document,
    //         "tmp" => Self::Document,
    //         "temp" => Self::Document,
    //
    //         "py" => Self::Code,
    //         "pyc" => Self::Code,
    //         "pyo" => Self::Code,
    //         "xml" => Self::Code,
    //         "html" => Self::Code,
    //         "htm" => Self::Code,
    //         "htmx" => Self::Code,
    //         "xhtml" => Self::Code,
    //         "xht" => Self::Code,
    //         "css" => Self::Code,
    //         "js" => Self::Code,
    //         "jsx" => Self::Code,
    //         "json" => Self::Code,
    //         "yaml" => Self::Code,
    //         "toml" => Self::Code,
    //         "ts" => Self::Code,
    //         "c" => Self::Code,
    //         "cpp" => Self::Code,
    //         "h" => Self::Code,
    //         "rs" => Self::Code,
    //         "r" => Self::Code,
    //         "go" => Self::Code,
    //         "zig" => Self::Code,
    //         "awk" => Self::Code,
    //         "cs" => Self::Code,
    //         "csproj" => Self::Code,
    //         "ici" => Self::Code,
    //         "ipynb" => Self::Code,
    //         "kt" => Self::Code,
    //         "lua" => Self::Code,
    //         "php" => Self::Code,
    //         "pl" => Self::Code,
    //         "pm" => Self::Code,
    //         "ps1" => Self::Code,
    //         "sh" => Self::Code,
    //         "fish" => Self::Code,
    //         "asm" => Self::Code,
    //         "d" => Self::Code,
    //         "vim" => Self::Code,
    //         "java" => Self::Code,
    //         "lisp" => Self::Code,
    //         "php3" => Self::Code,
    //         "php4" => Self::Code,
    //         "php5" => Self::Code,
    //         "phps" => Self::Code,
    //         "vb" => Self::Code,
    //         "sql" => Self::Code,
    //
    //         "exe" => Self::Executable,
    //         "apk" => Self::Executable,
    //         "o" => Self::Executable,
    //         "so" => Self::Executable,
    //         "app" => Self::Executable,
    //         "dll" => Self::Executable,
    //         "elf" => Self::Executable,
    //         "jar" => Self::Executable,
    //         "lib" => Self::Executable,
    //
    //         "mp3" => Self::Audio,
    //         "aiff" => Self::Audio,
    //         "aif" => Self::Audio,
    //         "aifc" => Self::Audio,
    //         "wav" => Self::Audio,
    //         "flac" => Self::Audio,
    //         "wma" => Self::Audio,
    //         "dts" => Self::Audio,
    //         "ac3" => Self::Audio,
    //         "aac" => Self::Audio,
    //         "ots" => Self::Audio,
    //         "ogg" => Self::Audio,
    //
    //         "gz" => Self::Archive,
    //         "gzip" => Self::Archive,
    //         "zst" => Self::Archive,
    //         "zstd" => Self::Archive,
    //         "zip" => Self::Archive,
    //         "7z" => Self::Archive,
    //         "7zip" => Self::Archive,
    //         "rar" => Self::Archive,
    //         "tar" => Self::Archive,
    //         "bin" => Self::Archive,
    //         "dat" => Self::Archive,
    //         "bz2" => Self::Archive,
    //         "pak" => Self::Archive,
    //         "par" => Self::Archive,
    //         "pax" => Self::Archive,
    //         "sqlite" => Self::Archive,
    //         "sq" => Self::Archive,
    //         "vbox" => Self::Archive,
    //
    //         "bam" => Self::GenomicData,
    //         "bai" => Self::GenomicData,
    //         "sam" => Self::GenomicData,
    //         "bed" => Self::GenomicData,
    //         "gtf" => Self::GenomicData,
    //         "gtf2" => Self::GenomicData,
    //         "gtf3" => Self::GenomicData,
    //         "gff" => Self::GenomicData,
    //         "gff2" => Self::GenomicData,
    //         "gff3" => Self::GenomicData,
    //         "bedpe" => Self::GenomicData,
    //         "cram" => Self::GenomicData,
    //         "sra" => Self::GenomicData,
    //         "fastq" => Self::GenomicData,
    //         "fasta" => Self::GenomicData,
    //         "fa" => Self::GenomicData,
    //         "fq" => Self::GenomicData,
    //         "fasterq" => Self::GenomicData,
    //         "embl" => Self::GenomicData,
    //         "genbank" => Self::GenomicData,
    //         "pdb" => Self::GenomicData,
    //         "ncbi" => Self::GenomicData,
    //         "maf" => Self::GenomicData,
    //         "nwk" => Self::GenomicData,
    //         "phd" => Self::GenomicData,
    //         "vcf" => Self::GenomicData,
    //         "pod5" => Self::GenomicData,
    //
    //         _ => Self::Other,
    //     }
    // }
}
