use std::path::PathBuf;

use clap::{command, value_parser, Arg};
use image::{GenericImageView, RgbaImage};
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{self, Options, TreeParsing};

const MASK: &[u8] = include_bytes!("../data/arch.svg");

fn main() {
    let arg = command!()
        .arg(
            Arg::new("INPUT")
                .help("Define background image to use")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("OUTPUT")
                .help("Define output file to write to")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("blur")
                .help("Blur the background image")
                .short('b')
                .long("blur")
                .value_parser(value_parser!(f32)),
        )
        .arg(
            Arg::new("darken")
                .help("Darken the background image")
                .short('d')
                .long("darken")
                .value_parser(value_parser!(i32)),
        )
        .get_matches();

    let inp_file = arg.get_one::<PathBuf>("INPUT").unwrap();
    let out_file = arg.get_one::<PathBuf>("OUTPUT").unwrap();

    println!("[*] Starting ArchPapers V{}", env!("CARGO_PKG_VERSION"));

    println!("[*] Loading `{}`", inp_file.to_string_lossy());
    let mut bg = match image::open(inp_file) {
        Ok(i) => i,
        Err(_) => return println!("[-] Invalid Image Input"),
    };

    // Blur
    if let Some(i) = arg.get_one::<f32>("blur") {
        println!("[*] Blurring Image");
        bg = bg.blur(*i);
    }

    // Darken
    if let Some(i) = arg.get_one::<i32>("darken") {
        println!("[*] Darking Image");
        bg = bg.brighten(-i);
    }

    println!("[*] Generating Image");
    let base = render_mask((bg.width(), bg.height()));
    image::imageops::overlay(&mut bg, &base, 0, 0);

    println!("[*] Saving Image to `{}`", out_file.to_string_lossy());
    if let Err(_) = bg.save(out_file) {
        println!("[-] Invalid Image Output");
    };
}

fn render_mask(size: (u32, u32)) -> RgbaImage {
    let svg = usvg::Tree::from_data(MASK, &Options::default()).unwrap();
    let svg = resvg::Tree::from_usvg(&svg);

    let scale_x = size.0 as f32 / svg.size.width();
    let scale_y = size.1 as f32 / svg.size.height();
    let scale = scale_x.min(scale_y);

    let mut pixmap = Pixmap::new(size.0, size.1).unwrap();
    svg.render(
        Transform::default().pre_scale(scale, scale),
        &mut pixmap.as_mut(),
    );

    image::RgbaImage::from_raw(size.0, size.1, pixmap.data().to_vec()).unwrap()
}
