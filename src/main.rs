use anyhow::{Context, Result};
use clap::Parser;
use las::{Read, Reader, Write, Writer};

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
    let header = reader.header().clone();
    let bounds = header.bounds();
    let mut las_points: Vec<las::Point> = Vec::new();

    for point in reader.points() {
        match point {
            Ok(point) => {
                if point.x > bounds.max.x
                    || point.y > bounds.max.y
                    || point.z > bounds.max.z
                    || point.x < bounds.min.x
                    || point.y < bounds.min.y
                    || point.z < bounds.min.z
                {
                    continue;
                }

                las_points.push(point)
            }
            Err(_) => {
                continue;
            }
        }
    }

    let mut writer = Writer::from_path(args.output, header).context("Failed to get las writer")?;

    for point in las_points {
        writer.write(point).context("Unable to write:")?;
    }

    writer.close().context("Failed to close las writer")?;

    Ok(())
}
