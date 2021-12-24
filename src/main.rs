use clap::{App, Arg};
use image;

const BASE: &[u8] = include_bytes!("../data/base.png");

fn main() {
    let arg = App::new("Archpapers")
        .version("1.0")
        .author("Connor Slade <connor@connorcode.com>")
        .about("Genarate Arch Linux Wallpapers")
        .arg(
            Arg::with_name("INPUT")
                .help("Define background image to use")
                .required(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Define output file to write to")
                .required(true),
        )
        .arg(
            Arg::with_name("blur")
                .help("Blur the background image")
                .short("b")
                .long("blur")
                .takes_value(true)
                .validator(|x| {
                    if x.parse::<f32>().is_err() {
                        return Err("Must be a vaild float (f32)".to_owned());
                    }
                    Ok(())
                }),
        )
        .arg(
            Arg::with_name("darken")
                .help("Darken the background image")
                .short("d")
                .long("darken")
                .takes_value(true)
                .validator(|x| {
                    if x.parse::<i32>().is_err() {
                        return Err("Must be a vaild intager (i32)".to_owned());
                    }
                    Ok(())
                }),
        )
        .get_matches();

    let inp_file = arg.value_of("INPUT").unwrap();
    let out_file = arg.value_of("OUTPUT").unwrap();

    println!("[*] Starting");
    let base = image::load_from_memory_with_format(BASE, image::ImageFormat::Png).unwrap();
    println!("[*] Loading `{}`", inp_file);
    let mut bg = match image::open(inp_file) {
        Ok(i) => i,
        Err(_) => return println!("[-] Invalid Image Input"),
    };

    // Blur
    if let Some(i) = arg.value_of("blur") {
        println!("[*] Bluring Image");
        bg = bg.blur(i.parse().unwrap());
    }

    // Darken
    if let Some(i) = arg.value_of("darken") {
        println!("[*] Darking Image");
        bg = bg.brighten(-i.parse::<i32>().unwrap());
    }

    println!("[*] Genarateing Image");
    let bg_dim = bg.clone().into_rgba8().dimensions();
    let base = base.resize_to_fill(bg_dim.0, bg_dim.1, image::imageops::Triangle);

    image::imageops::overlay(&mut bg, &base, 0, 0);

    println!("[*] Saveing Image to `{}`", out_file);
    match bg.save(out_file) {
        Ok(_) => {}
        Err(_) => return println!("[-] Invalid Image Output"),
    };
}
