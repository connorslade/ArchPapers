# 🚀 ArchPapers-CLI

## Installation

### Build From Source

1. [Install Rust & Cargo](https://rustup.rs)
2. Build The Crate with the following command

```bash
cargo install arch_papers
```

### Download a Binary

Head over to [Releases](https://github.com/Basicprogrammer10/ArchPapers/releases) and download the executable.

Bonus points if you verify it with GPG.
My key is [here](https://connorcode.com/key.asc).

## Usage

Running with the flag `--help` will show the following help message

```plain
Generate Arch Linux Wallpapers

Usage: arch_papers.exe [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   The image to use as the background / foreground
  <OUTPUT>  The output file to write to

Options:
  -i, --invert                   Invert the mask. By using this option, your supplied image will be used as the foreground
  -o, --original                 Whether to use the original image as background
  -t, --translate <TRANSLATE>    The translation to apply to the background image. (x, y) [default: 0,0]
  -m, --mask <MASK>              Lets you use your own mask, not just the arch logo. The file MUST be a SVG with the alpha channel being used as the mask
  -S, --mask-scale <MASK_SCALE>  The scale to apply to the mask. Useful for use with custom masks [default: 1.0]
      --bg-scale <BG_SCALE>      [default: 1.0]
  -s, --size <SIZE>              The size of the output image. By default, this is the same as the input image. (width, height)
  -b, --blur <BLUR>              How much to blur the background image. None by default
  -d, --darken <DARKEN>          How much to darken the background image. None by default
  -h, --help                     Print help
  -V, --version                  Print version
```

Example usage:

```plain
arch_papers image.png archified.png
```

Or to blur, dim, and translate the background image

```plain
arch_papers -b 10 -d 10 -t 1240,-80 image.png archified.png
```

## Examples

Here are some examples of wallpapers generated with ArchPapers - that don't use the the default arch logo mask.

| <img src="https://github.com/Basicprogrammer10/ArchPapers/assets/50306817/7b4211ba-cede-4fcb-b5a6-ee42c62ee745" />                                                                                                                                                | <img src="https://github.com/Basicprogrammer10/ArchPapers/assets/50306817/8373d8eb-d4af-4403-80b8-d5861214cdf7" />                                                                                                                                                    |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Background with the [rust-lang logo](https://www.rust-lang.org) and [Nautical](https://www.reddit.com/r/wallpapers/comments/tkhpx9/nautical_3840x2160) from James Sypniewski.<br>`arch-papers nautical.png out.png --mask rust.svg -s 3840,2160 -S 0.5 -t 1000,0` | Background with the [regolith DE](https://regolith-desktop.com) logo and [Dreams](https://www.reddit.com/r/wallpaper/comments/m7x6a4/dreams_3440x1440) from James Sypniewski.<br>`arch-papers dreams.png out.png --mask regolith.svg -s 3840,2160 -S 0.5 -t 1055,140` |
