#![deny(clippy::unwrap_used)]
use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use clap::Parser;
use image::{imageops, DynamicImage, GenericImageView};
use log::info;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Directory containing source images
    #[arg(long, env = "INPUT_SOURCE_DIR")]
    source_dir: PathBuf,

    /// Comma separated list of widths
    #[arg(long, env = "INPUT_SIZES")]
    sizes: String,

    /// Output directory
    #[arg(long, env = "INPUT_OUTPUT_DIR")]
    output_dir: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    run(args)
}

fn run(args: Args) -> Result<()> {
    let sizes = parse_sizes(&args.sizes)?;
    fs::create_dir_all(&args.output_dir)
        .with_context(|| format!("creating output dir {}", args.output_dir.display()))?;

    for entry in fs::read_dir(&args.source_dir)
        .with_context(|| format!("reading source dir {}", args.source_dir.display()))?
    {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }
        if ensure_supported(&path).is_err() {
            continue;
        }
        let img = load_image(&path)?;
        for size in &sizes {
            process_size(&img, &path, &args.output_dir, *size)?;
        }
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

fn process_size(img: &DynamicImage, src_path: &Path, out_dir: &Path, size: u32) -> Result<()> {
    let (w, h) = img.dimensions();
    let new_h = (h as f32 * size as f32 / w as f32).round() as u32;
    let resized = imageops::resize(img, size, new_h, imageops::FilterType::Lanczos3);

    let base = src_path.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
    let ext = src_path.extension().and_then(|s| s.to_str()).unwrap_or("png");
    let out_path = out_dir.join(format!("{}-{}{}.{}", base, size, "", ext));
    if out_path.exists() {
        info!("Skipping {}, already exists", out_path.display());
        return Ok(());
    }
    resized.save(&out_path).with_context(|| format!("saving {}", out_path.display()))?;
    info!("Resized {} to width {} -> {}", src_path.display(), size, out_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sizes_basic() {
        let sizes = parse_sizes("64,128, 64").expect("valid size list");
        assert_eq!(sizes, vec![64, 128]);
    }

    #[test]
    fn validate_supported() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let file = dir.path().join("a.txt");
        fs::write(&file, "hi").expect("write test file");
        assert!(ensure_supported(&file).is_err());
    }

    #[test]
    fn resize_basic() {
        let tmp = tempfile::tempdir().expect("create temp dir");
        let src_path = tmp.path().join("test.png");
        DynamicImage::new_rgb8(8, 8).save(&src_path).expect("save source image");
        let img = load_image(&src_path).expect("load image");
        let out_dir = tempfile::tempdir().expect("create temp dir");
        process_size(&img, &src_path, out_dir.path(), 4).expect("resize image");
        let out = out_dir.path().join("test-4.png");
        assert!(out.exists());
    }
}
