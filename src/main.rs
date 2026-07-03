use anyhow::{Context, Result};
use clap::Parser;
use las::{Builder, Read, Reader, Write, Writer};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long, default_value_t = String::from("./"))]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut reader = Reader::from_path(args.path).context("Failed to open las file")?;

    let original_header = reader.header().clone();

    let mut header = Builder::from(original_header).into_header()?;

    let mut las_points: Vec<las::Point> = Vec::new();
    let mut dropped: u64 = 0;

    for point in reader.points() {
        match point {
            Ok(point) => {
                header.add_point(&point);
                las_points.push(point);
            }
            Err(_) => {
                dropped += 1;
                continue;
            }
        }
    }

    let kept = las_points.len();

    let mut writer = Writer::from_path(args.output, header).context("Failed to get las writer")?;

    for point in las_points {
        writer.write(point).context("Unable to write:")?;
    }

    writer.close().context("Failed to close las writer")?;

    eprintln!("Wrote {kept} points, dropped {dropped} invalid points");

    Ok(())
}
