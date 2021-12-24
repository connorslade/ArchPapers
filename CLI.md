# 🚀 ArchPapers-CLI

## Installation
### Build From Source
1. [Install Rust & Cargo](https://rustup.rs/)
2. Build The Crate with the following command
```bash
$ cargo install arch_papers
```

### Download a Binary
Head over to [Releases](https://github.com/Basicprogrammer10/ArchPapers/releases) and download the executable.

Bonus points if you verify it with GPG. My key is [here](https://connorcode.com/key.asc).

## Usage
Running with the flag `--help` will show the following help message
```
Archpapers 1.0
Connor Slade <connor@connorcode.com>
Genarate Arch Linux Wallpapers

USAGE:
    arch_papers [OPTIONS] <INPUT> <OUTPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --blur <blur>        Blur the background image
    -d, --darken <darken>    Darken the background image

ARGS:
    <INPUT>     Define background image to use
    <OUTPUT>    Define output file to write to
```

Example usage:
```
arch_papers image.png archified.png
```

Or to blur and dim the background image
```
arch_papers -b 10 -d 10 image.png archified.png
```
