use std::{fs, path::{Path, PathBuf}};

use anyhow::{Context, Result};
use clap::Parser;
use image::{imageops, DynamicImage, GenericImageView, RgbImage, RgbaImage, Rgb, Rgba};
use log::info;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the source image
    #[arg(long, env = "INPUT_IMAGE_PATH")]
    image_path: PathBuf,

    /// Comma separated list of widths
    #[arg(long, env = "INPUT_SIZES")]
    sizes: String,

    /// Output directory
    #[arg(long, env = "INPUT_OUTPUT_DIR")]
    output_dir: PathBuf,

    /// Crop instead of pad
    #[arg(long, env = "INPUT_CROP", default_value_t = false)]
    crop: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    run(args)
}

fn run(args: Args) -> Result<()> {
    info!("Loading image..." );
    let img = load_image(&args.image_path)?;
    let sizes = parse_sizes(&args.sizes)?;
    fs::create_dir_all(&args.output_dir)
        .with_context(|| format!("creating output dir {}", args.output_dir.display()))?;

    for size in sizes {
        process_size(&img, &args.image_path, &args.output_dir, size, args.crop)?;
    }
    Ok(())
}

fn load_image(path: &Path) -> Result<DynamicImage> {
    ensure_supported(path)?;
    let img = image::open(path).with_context(|| format!("opening {}", path.display()))?;
    Ok(img)
}

fn ensure_supported(path: &Path) -> Result<()> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_default();
    let supported = ["jpg", "jpeg", "png", "gif", "bmp"];
    if !supported.contains(&ext.as_str()) {
        anyhow::bail!("Unsupported image format: {}", ext);
    }
    if !path.exists() {
        anyhow::bail!("Image path does not exist: {}", path.display());
    }
    Ok(())
}

fn parse_sizes(s: &str) -> Result<Vec<u32>> {
    let mut out = Vec::new();
    for part in s.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let value: u32 = part.parse().with_context(|| format!("invalid size '{}'", part))?;
        if !out.contains(&value) {
            out.push(value);
        }
    }
    Ok(out)
}

fn process_size(img: &DynamicImage, src_path: &Path, out_dir: &Path, size: u32, crop: bool) -> Result<()> {
    let (w, h) = img.dimensions();
    let scale = size as f32 / w.min(h) as f32;
    let new_w = (w as f32 * scale).round() as u32;
    let new_h = (h as f32 * scale).round() as u32;
    let resized = imageops::resize(img, new_w, new_h, imageops::FilterType::Lanczos3);

    let final_img = if crop {
        let x = if new_w > size { (new_w - size) / 2 } else { 0 };
        let y = if new_h > size { (new_h - size) / 2 } else { 0 };
        let cropped = imageops::crop_imm(&resized, x, y, size, size).to_image();
        DynamicImage::ImageRgba8(cropped)
    } else {
        let mut canvas = if needs_alpha(src_path) {
            DynamicImage::ImageRgba8(RgbaImage::from_pixel(size, size, Rgba([0, 0, 0, 0])))
        } else {
            DynamicImage::ImageRgb8(RgbImage::from_pixel(size, size, Rgb([255, 255, 255])))
        };
        let x = ((size as i64 - new_w as i64) / 2).max(0);
        let y = ((size as i64 - new_h as i64) / 2).max(0);
        imageops::overlay(&mut canvas, &resized, x, y);
        canvas
    };

    let base = src_path.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
    let ext = src_path.extension().and_then(|s| s.to_str()).unwrap_or("png");
    let out_path = out_dir.join(format!("{}-{}x{}.{}", base, size, size, ext));
    final_img
        .save(&out_path)
        .with_context(|| format!("saving {}", out_path.display()))?;
    info!(
        "Resizing to {}x{} (crop={})... saved to {}",
        size,
        size,
        crop,
        out_path.display()
    );
    Ok(())
}

fn needs_alpha(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()).map(|e| e.to_ascii_lowercase()) {
        Some(ref ext) if ext == "png" || ext == "gif" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sizes_basic() {
        let sizes = parse_sizes("64,128, 64").unwrap();
        assert_eq!(sizes, vec![64, 128]);
    }

    #[test]
    fn validate_supported() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("a.txt");
        fs::write(&file, "hi").unwrap();
        assert!(ensure_supported(&file).is_err());
    }

    #[test]
    fn resize_basic() {
        let tmp = tempfile::tempdir().unwrap();
        let src_path = tmp.path().join("test.png");
        DynamicImage::new_rgb8(8, 8).save(&src_path).unwrap();
        let img = load_image(&src_path).unwrap();
        let out_dir = tempfile::tempdir().unwrap();
        process_size(&img, &src_path, out_dir.path(), 4, false).unwrap();
        let out = out_dir.path().join("test-4x4.png");
        assert!(out.exists());
    }
}

