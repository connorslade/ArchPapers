use std::{path::PathBuf, str::FromStr};

use clap::Parser;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Args {
    // == General Options ==
    /// The image to use as the background / foreground.
    #[arg()]
    pub input: PathBuf,

    /// The output file to write to.
    #[arg()]
    pub output: PathBuf,

    // == Mask Options ==
    /// Invert the mask.
    /// By using this option, your supplied image will be used as the foreground.
    #[arg(short, long)]
    pub invert: bool,

    /// The color to use for the foreground.
    #[arg(short, long, value_parser = hex, default_value = "#171718")]
    pub color: [u8; 3],

    /// The translation to apply to the background image.
    /// (x, y)
    #[arg(short, long, value_parser = coords::<i32>, default_value = "0,0")]
    pub translate: (i32, i32),

    /// Lets you use your own mask, not just the arch logo.
    /// The file MUST be a SVG with the alpha channel being used as the mask.
    #[arg(short, long, value_parser = svg)]
    pub mask: Option<PathBuf>,

    /// The scale to apply to the mask.
    /// Useful for use with custom masks.
    #[arg(short = 'S', long, default_value = "1.0")]
    pub mask_scale: f32,

    // The scale to apply to the background image.
    // Useful for use with custom masks.
    #[arg(long, default_value = "1.0")]
    pub bg_scale: f32,

    /// The size of the output image.
    /// By default, this is the same as the input image.
    /// (width, height)
    #[arg(short, long, value_parser = coords::<u32>)]
    pub size: Option<(u32, u32)>,

    // == Image Effects ==
    /// How much to blur the background image.
    /// None by default.
    #[arg(short, long)]
    pub blur: Option<f32>,

    /// How much to darken the background image.
    /// None by default.
    #[arg(short, long)]
    pub darken: Option<i32>,
}

fn hex(inp: &str) -> Result<[u8; 3], String> {
    let inp = inp.strip_prefix('#').unwrap_or(inp);
    if inp.len() != 6 {
        return Err("Invalid Hex Color".into());
    }

    let mut out = [0; 3];
    for (i, c) in inp.chars().enumerate() {
        let c = c.to_digit(16).ok_or("Invalid Hex Color")?;
        out[i / 2] = (out[i / 2] << 4) | c as u8;
    }

    Ok(out)
}

fn coords<T: FromStr>(inp: &str) -> Result<(T, T), String> {
    let mut inp = inp.split(',');
    let x = inp
        .next()
        .ok_or("Invalid Coordinates")?
        .parse::<T>()
        .map_err(|_| "Invalid Coordinates")?;
    let y = inp
        .next()
        .ok_or("Invalid Coordinates")?
        .parse::<T>()
        .map_err(|_| "Invalid Coordinates")?;

    Ok((x, y))
}

fn svg(inp: &str) -> Result<PathBuf, String> {
    let inp = PathBuf::from(inp);

    if !inp.exists() {
        return Err("Path does not exist".into());
    }

    if !inp.is_file() {
        return Err("Path is not a file".into());
    }

    if inp.extension().unwrap_or_default() != "svg" {
        return Err("Path is not a SVG".into());
    }

    Ok(inp)
}
