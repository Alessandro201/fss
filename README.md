# File Size Summary

Show the file sizes grouped by extension, file type (Image, Video, Document, ...), parent directory or file name.

Example of running `fss` on the repo (after some builds):
```bash
$ fss -g type
   1.56 MB	Code
   4.92 MB	Image
  16.67 MB	Archive
  28.25 MB	Executable
 576.12 MB	Other

Total:   
 627.52 MB
```

## Usage
```bash
Computes disk-usage for the given entries and groups them by extension or file types
Usage: fss [OPTIONS] [INPUTS]...
Arguments:
  [INPUTS]...
          List of paths
          
          [default: .]
Options:
  -g, --group-by <GROUP_BY>
          Select how to group the files sizes. [values: e, t, f, d]
          
              'e': extension
              't': file type, eg. Images, Videos, Documents...
              'f': file name
              'd': parent directory
          
          [default: extension]
  -S, --size <SIZE>
          Limit results based on the size of files using the format <+-><NUM><UNIT>.
             '+': file size must be greater than or equal to this
             '-': file size must be less than or equal to this
          
          If neither '+' nor '-' is specified, file size must be exactly equal to this.
             'NUM':  The numeric size (e.g. 500)
             'UNIT': The units for NUM. They are not case-sensitive.
          Allowed unit values:
              'b':  bytes
              'k':  kilobytes (base ten, 10^3 = 1000 bytes)
              'm':  megabytes
              'g':  gigabytes
              't':  terabytes
              'ki': kibibytes (base two, 2^10 = 1024 bytes)
              'mi': mebibytes
              'gi': gibibytes
              'ti': tebibytes
  -s, --size-format <SIZE_FORMAT>
          Output format for file sizes (decimal: base-10 MB, binary: base 2 MiB, bytes: raw byte count B)
          
          [default: decimal]
          [possible values: decimal, binary, bytes]
  -b, --apparent-size
          Compute apparent size instead of disk usage
  -j, --threads <THREADS>
          Set the number of threads to use. Default 3 x num cores
          
          [default: 48]
  -v, --verbose
          Do not hide filesystem errors
  -h, --help
          Print help (see a summary with '-h')
  -V, --version
          Print version


```

### FileType
The available filetype are:
  - Image
  - Video
  - Document
  - Executable
  - Archive
  - Audio
  - Code
  - GenomicData
  - Other

This is the full list of extension per file type

### Image
  - avif
  - bmp
  - dng
  - gif
  - heic
  - jpeg
  - jpegxl
  - jpg
  - nef
  - png
  - psd
  - raw
  - svg
  - tiff
  - webp

### Video

### Document

### Executable

### Archive

### Audio

### Code

### GenomicData

### Other



# Disclaimer & Citations
The code for this app was adapted from [diskus](https://github.com/sharkdp/diskus) by sharkdp. Furthermore the size filtering was taken from [fd](https://github.com/sharkdp/fd) by the same author.
He makes some really great programs, go check them out!
