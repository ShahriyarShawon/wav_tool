mod wav;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use wav::WavFile;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String,

    #[arg(short, long)]
    output_file: String,

    #[arg(short, long)]
    length: u32,

    #[arg(long, default_value_t = 0)]
    offset: u32,
}


fn main() {
    let args = Args::parse();
    let filename: &str = &args.input_file;

    let mut file = File::open(filename).expect("Where the file at broski");
    let mut wav_file = WavFile::from_file(&mut file);
    file.seek(std::io::SeekFrom::Start(42))
        .expect("Could not seek that far");
    let new_wav_file = wav_file.get_slice_as_wav(&mut file, args.length, args.offset);
    new_wav_file.write_wav_to_file(&args.output_file);
}
