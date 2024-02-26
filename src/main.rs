use std::{borrow::Cow, fs};

use clap::Parser;
use image::{DynamicImage, RgbaImage};
use tiny_skia::{Color, IntSize, Mask, MaskType, Pixmap, PremultipliedColorU8, Transform};
use usvg::{Options, TreeParsing};

mod args;

const MASK: &[u8] = include_bytes!("../data/arch.svg");

fn main() {
    let arg = args::Args::parse();
    println!("[*] Starting ArchPapers V{}", env!("CARGO_PKG_VERSION"));

    println!("[*] Loading `{}`", arg.input.to_string_lossy());
    let mut bg = match image::open(arg.input) {
        Ok(i) => i,
        Err(_) => return println!("[-] Invalid Image Input"),
    };

    // Create foreground mask
    let size = arg.size.unwrap_or((bg.width(), bg.height()));
    let mut colored = if arg.original {
        if arg.size.is_some() {
            println!("[-] Using --size contradicts --original, the size will be ignored");
        }
        Pixmap::from_vec(
            bg.to_rgba8().into_vec(),
            IntSize::from_wh(bg.width(), bg.height()).expect("Size too big"),
        )
        .unwrap()
    } else {
        let mut single_color_bg = Pixmap::new(size.0, size.1).unwrap();
        single_color_bg.fill(Color::from_rgba8(
            arg.color[0],
            arg.color[1],
            arg.color[2],
            255,
        ));
        single_color_bg
    };

    // Translate
    bg = DynamicImage::from(imageproc::geometric_transformations::translate(
        &bg.to_rgb8(),
        (arg.translate.0, arg.translate.1),
    ));

    // Blur
    if let Some(i) = arg.blur {
        println!("[*] Blurring Image");
        bg = DynamicImage::from(imageproc::filter::gaussian_blur_f32(&bg.to_rgb8(), i));
    }

    // Darken
    if let Some(i) = arg.darken {
        println!("[*] Darking Image");
        bg = bg.brighten(-i);
    }

    println!("[*] Generating Image");

    // Load mask from SVG
    let mask_data = match arg.mask {
        Some(m) => Cow::Owned(fs::read(m).expect("Error reading mask")),
        None => Cow::Borrowed(MASK),
    };
    let mask = render_mask(mask_data, size, arg.mask_scale, arg.invert);

    colored.apply_mask(&mask);

    // Scale background image
    if arg.bg_scale != 1.0 {
        bg = bg.resize(
            (bg.width() as f32 * arg.bg_scale) as u32,
            (bg.height() as f32 * arg.bg_scale) as u32,
            image::imageops::FilterType::Lanczos3,
        );
    }

    let overlay_x = (size.0 as i64 - bg.width() as i64) / 2;
    let overlay_y = (size.1 as i64 - bg.height() as i64) / 2;

    // Composite images
    let mut image = image::RgbaImage::from_pixel(size.0, size.1, image::Rgba([0, 0, 0, 255]));
    let colored_img = RgbaImage::from_raw(size.0, size.1, colored.data().to_vec()).unwrap();
    image::imageops::overlay(&mut image, &bg, overlay_x, overlay_y);
    image::imageops::overlay(&mut image, &colored_img, 0, 0);

    println!("[*] Saving Image to `{}`", arg.output.to_string_lossy());
    if let Err(e) = image.save(arg.output) {
        println!("[-] Error saving image\n{e}");
    };
}

fn render_mask(mask: Cow<[u8]>, size: (u32, u32), scale: f32, invert: bool) -> Mask {
    let svg = usvg::Tree::from_data(&mask, &Options::default()).unwrap();
    let svg = resvg::Tree::from_usvg(&svg);

    let scale_x = size.0 as f32 / svg.size.width();
    let scale_y = size.1 as f32 / svg.size.height();
    let scale = scale_x.min(scale_y) * scale;

    let translate_x = (size.0 as f32 - svg.size.width() * scale) / 2.0;
    let translate_y = (size.1 as f32 - svg.size.height() * scale) / 2.0;

    let mut pixmap = Pixmap::new(size.0, size.1).unwrap();
    svg.render(
        Transform::default()
            .pre_scale(scale, scale)
            .post_translate(translate_x, translate_y),
        &mut pixmap.as_mut(),
    );

    if !invert {
        pixmap.pixels_mut().iter_mut().for_each(|p| {
            *p = PremultipliedColorU8::from_rgba(0, 0, 0, 255 - p.alpha()).unwrap();
        });
    }

    Mask::from_pixmap(pixmap.as_ref(), MaskType::Alpha)
}
