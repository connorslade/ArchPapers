use clap::Parser;
use image::{GenericImageView, RgbaImage};
use resvg::tiny_skia::{Color, Mask, MaskType, Pixmap, PremultipliedColorU8, Transform};
use resvg::usvg::{self, Options, TreeParsing};

mod args;

const MASK: &[u8] = include_bytes!("../data/arch.svg");

fn main() {
    println!("[*] Starting ArchPapers V{}", env!("CARGO_PKG_VERSION"));
    let arg = args::Args::parse();

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
    let size = (bg.width(), bg.height());
    let mask = render_mask(size, arg.invert);

    let mut colored = Pixmap::new(size.0, size.1).unwrap();
    colored.fill(Color::from_rgba8(
        arg.color[0],
        arg.color[1],
        arg.color[2],
        255,
    ));
    colored.apply_mask(&mask);
    let colored_img = RgbaImage::from_raw(size.0, size.1, colored.data().to_vec()).unwrap();
    image::imageops::overlay(&mut bg, &colored_img, 0, 0);

    println!("[*] Saving Image to `{}`", arg.output.to_string_lossy());
    if let Err(e) = bg.save(arg.output) {
        println!("[-] Error saving image\n{e}");
    };
}

fn render_mask(size: (u32, u32), invert: bool) -> Mask {
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

    if !invert {
        pixmap.pixels_mut().iter_mut().for_each(|p| {
            *p = PremultipliedColorU8::from_rgba(0, 0, 0, 255 - p.alpha()).unwrap();
        });
    }

    Mask::from_pixmap(pixmap.as_ref(), MaskType::Alpha)
}
