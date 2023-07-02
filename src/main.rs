use clap::Parser;
use image::{GenericImageView, RgbaImage};
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{self, Options, TreeParsing};

mod args;

const MASK: &[u8] = include_bytes!("../data/arch.svg");

fn main() {
    println!("[*] Starting ArchPapers V{}", env!("CARGO_PKG_VERSION"));
    let arg = args::Args::parse();
    dbg!(&arg.color);

    println!("[*] Loading `{}`", arg.input.to_string_lossy());
    let mut bg = match image::open(arg.input) {
        Ok(i) => i,
        Err(_) => return println!("[-] Invalid Image Input"),
    };

    // Blur
    if let Some(i) = arg.blur {
        println!("[*] Blurring Image");
        bg = bg.blur(i);
    }

    // Darken
    if let Some(i) = arg.darken {
        println!("[*] Darking Image");
        bg = bg.brighten(-i);
    }

    println!("[*] Generating Image");
    let base = render_mask((bg.width(), bg.height()));
    image::imageops::overlay(&mut bg, &base, 0, 0);

    println!("[*] Saving Image to `{}`", arg.output.to_string_lossy());
    if let Err(e) = bg.save(arg.output) {
        println!("[-] Error saving image\n{e}");
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
