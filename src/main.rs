use clap::Parser;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use std::path::Path;

#[derive(Parser)]
struct Cli {
    crown: String,
    input: String,
    output: String,
}

fn main() {
    let args = Cli::parse();

    // 画像を読み込む
    let mut img = image::open(&Path::new(&args.input)).expect("Failed to open input image");
    let crown = image::open(&Path::new(&args.crown)).expect("Failed to open input image");

    // 王冠画像をリサイズする
    let (width, height) = img.dimensions();
    let crown_resized = crown.resize(width / 4, height / 4, image::imageops::FilterType::Lanczos3);

    // 王冠画像を入力画像に重ねる
    let (crown_width, crown_height) = crown_resized.dimensions();
    let x_offset = (width - crown_width) / 2;
    let y_offset = height - crown_height;
    overlay(&mut img, &crown_resized, x_offset, y_offset);

    // 画像を保存する
    img.save(&Path::new(&args.output)).expect("Failed to save output image");
}


fn overlay(base: &mut DynamicImage, overlay: &DynamicImage, x: u32, y: u32) {
    for oy in 0..overlay.height() {
        for ox in 0..overlay.width() {
            let overlay_pixel = overlay.get_pixel(ox, oy);
            let base_pixel = base.get_pixel(x + ox, y + oy);

            let alpha = overlay_pixel[3] as f32 / 255.0;
            let inv_alpha = 1.0 - alpha;

            let blended_pixel = Rgba([
                (overlay_pixel[0] as f32 * alpha + base_pixel[0] as f32 * inv_alpha) as u8,
                (overlay_pixel[1] as f32 * alpha + base_pixel[1] as f32 * inv_alpha) as u8,
                (overlay_pixel[2] as f32 * alpha + base_pixel[2] as f32 * inv_alpha) as u8,
                (overlay_pixel[3] as f32 * alpha + base_pixel[3] as f32 * inv_alpha) as u8,
            ]);

            base.put_pixel(x + ox, y + oy, blended_pixel);
        }
    }
}