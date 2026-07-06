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

    let mut reader = Reader::from_path(&args.path).context("Failed to open las file")?;
    let header = reader.header().clone();
    let mut writer = Writer::from_path(&args.output, header).context("Failed to create las writer")?;

    for point in reader.points() {
        match point {
            Ok(point) => writer.write(point).context("Failed to write point")?,
            Err(_) => continue,
        }
    }

    writer.close().context("Failed to close las writer")?;

    Ok(())
}
