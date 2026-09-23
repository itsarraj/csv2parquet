use anyhow::{Context, Result};
use arrow::csv;
use arrow::csv::reader::Format;
use clap::Parser;
use parquet::arrow::ArrowWriter;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input CSV file
    input: PathBuf,

    /// Output Parquet file
    output: PathBuf,

    /// Does the CSV file have a header?
    #[arg(short, long, default_value_t = true)]
    has_header: bool,

    /// Batch size for reading
    #[arg(short, long, default_value_t = 1024)]
    batch_size: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut file =
        File::open(&args.input).with_context(|| format!("Failed to open {:?}", args.input))?;

    // Infer schema (reading up to 100 records)
    let format = Format::default().with_header(args.has_header);
    let (schema, _) = format
        .infer_schema(&mut file, Some(100))
        .context("Failed to infer schema")?;

    let schema_ref = Arc::new(schema);

    // Re-open for actual reading
    let file = File::open(&args.input)?;
    let reader = csv::ReaderBuilder::new(schema_ref.clone())
        .with_format(format)
        .with_batch_size(args.batch_size)
        .build(file)
        .context("Failed to build CSV reader")?;

    let out_file = File::create(&args.output)
        .with_context(|| format!("Failed to create {:?}", args.output))?;

    let mut writer = ArrowWriter::try_new(out_file, schema_ref, None)
        .context("Failed to create Parquet writer")?;

    let mut record_count = 0;

    for batch in reader.into_iter() {
        let batch = batch.context("Error reading CSV batch")?;
        record_count += batch.num_rows();
        writer
            .write(&batch)
            .context("Error writing Parquet batch")?;
    }

    writer.close().context("Failed to close Parquet writer")?;

    println!("Successfully converted {} rows to Parquet.", record_count);

    Ok(())
}
