use std::path::PathBuf;

use clap::{command, value_parser, Arg};

const BASE: &[u8] = include_bytes!("../data/base.png");

fn main() {
    let arg = command!()
        // .version(VERSION)
        // .author("Connor Slade <connor@connorcode.com>")
        // .about("Generate Arch Linux Wallpapers")
        .arg(
            Arg::new("INPUT")
                .help("Define background image to use")
                .required(true),
        )
        .arg(
            Arg::new("OUTPUT")
                .help("Define output file to write to")
                .required(true),
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
    let base = image::load_from_memory_with_format(BASE, image::ImageFormat::Png).unwrap();
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
    let bg_dim = bg.clone().into_rgba8().dimensions();
    let base = base.resize_to_fill(bg_dim.0, bg_dim.1, image::imageops::Triangle);

    image::imageops::overlay(&mut bg, &base, 0, 0);

    println!("[*] Saving Image to `{}`", out_file.to_string_lossy());
    if let Err(_) = bg.save(out_file) {
        println!("[-] Invalid Image Output");
    };
}
