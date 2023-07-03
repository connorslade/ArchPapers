use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// The image to use as the background / foreground.
    #[arg()]
    pub input: PathBuf,

    /// The output file to write to.
    #[arg()]
    pub output: PathBuf,

    /// Invert the mask.
    /// By using this option, your supplied image will be used as the foreground.
    #[arg(short, long)]
    pub invert: bool,

    /// The color to use for the background
    #[arg(short, long, value_parser = hex, default_value = "#171718")]
    pub color: [u8; 3],

    /// How much to blur the background image.
    /// None by default.
    #[arg(short, long)]
    pub blur: Option<f32>,

    /// How much to darken the background image.
    /// None by default.
    #[arg(short, long)]
    pub darken: Option<i32>,

    /// The translation to apply to the background image.
    /// (x, y)
    #[arg(short, long, value_parser = coords, default_value = "0,0")]
    pub translate: (i32, i32),
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

fn coords(inp: &str) -> Result<(i32, i32), String> {
    let mut inp = inp.split(',');
    let x = inp
        .next()
        .ok_or("Invalid Coordinates")?
        .parse::<i32>()
        .map_err(|_| "Invalid Coordinates")?;
    let y = inp
        .next()
        .ok_or("Invalid Coordinates")?
        .parse::<i32>()
        .map_err(|_| "Invalid Coordinates")?;

    Ok((x, y))
}
