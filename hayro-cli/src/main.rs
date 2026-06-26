use clap::Parser;
use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::{RenderCache, RenderSettings, render, vello_cpu};
use std::path::PathBuf;

/// Render PDF pages to PNG, with optional auto-crop and resize.
#[derive(Parser)]
#[command(name = "hayro", version)]
struct Cli {
    /// Input PDF file
    input: PathBuf,

    /// Output PNG file (use {n} for page number, e.g. page-{n}.png)
    #[arg(short, long, default_value = "output.png")]
    output: String,

    /// Scale factor (default: 4.0 for ~300 DPI)
    #[arg(short, long, default_value = "4.0")]
    scale: f32,

    /// Auto-crop white borders
    #[arg(long)]
    autocrop: bool,

    /// Target width in pixels (requires --autocrop)
    #[arg(long, default_value = "1200")]
    width: u32,
}

fn main() {
    let cli = Cli::parse();

    let data = std::fs::read(&cli.input).expect("read pdf");
    let pdf = Pdf::new(data).expect("parse pdf");
    let pages = pdf.pages();

    let cache = RenderCache::new();
    let settings = InterpreterSettings::default();

    for (i, page) in pages.iter().enumerate() {
        let pixmap = render(
            page,
            &cache,
            &settings,
            &RenderSettings {
                x_scale: cli.scale,
                y_scale: cli.scale,
                bg_color: vello_cpu::color::palette::css::WHITE,
                ..Default::default()
            },
        );

        let raw_png = pixmap.into_png().expect("encode png");

        let final_png = if cli.autocrop {
            hayro::auto_crop_png(&raw_png, Some(cli.width))
        } else {
            raw_png
        };

        let out = cli.output.replace("{n}", &i.to_string());
        std::fs::write(&out, &final_png).expect("write png");
        println!("OK {}", out);
    }
}
