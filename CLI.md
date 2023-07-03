# 🚀 ArchPapers-CLI

## Installation

### Build From Source

1. [Install Rust & Cargo](https://rustup.rs)
2. Build The Crate with the following command

```bash
$ cargo install arch_papers
```

### Download a Binary

Head over to [Releases](https://github.com/Basicprogrammer10/ArchPapers/releases) and download the executable.

Bonus points if you verify it with GPG.
My key is [here](https://connorcode.com/key.asc).

## Usage

Running with the flag `--help` will show the following help message

```
Generate Arch Linux Wallpapers

Usage: arch_papers.exe [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   The image to use as the background / foreground
  <OUTPUT>  The output file to write to

Options:
  -i, --invert                 Invert the mask. By using this option, your supplied image will be used as the foreground
  -c, --color <COLOR>          The color to use for the background [default: #171718]
  -b, --blur <BLUR>            How much to blur the background image. None by default
  -d, --darken <DARKEN>        How much to darken the background image. None by default
  -t, --translate <TRANSLATE>  The translation to apply to the background image. (x, y) [default: 0,0]
  -h, --help                   Print help
  -V, --version                Print version
```

Example usage:

```
arch_papers image.png archified.png
```

Or to blur, dim, and translate the background image

```
arch_papers -b 10 -d 10 -t 1240,-80 image.png archified.png
```
