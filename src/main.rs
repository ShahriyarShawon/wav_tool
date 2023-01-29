use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

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

#[derive(Debug, Default)]
#[repr(C)]
struct WavFile {
    chunkid: [u8; 4],
    chunksize: u32,
    format: [u8; 4],
    subchunk1id: [u8; 4],
    subchunk1size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_alignment: u16,
    bits_per_sample: u16,
    subchunk2id: [u8; 4],
    subchunk2size: u32,
    data: Vec<u8>,
}

impl WavFile {
    #[allow(dead_code)]
    fn print_wav_file(&self) -> () {
        println!(
            "{:?}",
            std::str::from_utf8(&self.chunkid).expect("Could not convert to string")
        );
        println!(
            "{:?}",
            std::str::from_utf8(&self.format).expect("Could not convert to string")
        );
        println!(
            "{:?}",
            std::str::from_utf8(&self.subchunk1id).expect("Could not convert to string")
        );
        println!(
            "{:?}",
            std::str::from_utf8(&self.subchunk2id).expect("Could not convert to string")
        );
    }
}

unsafe fn struct_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    return std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>());
}

fn main() {
    let args = Args::parse();
    //let args: Vec<String> = env::args().collect();
    let filename: &str = &args.input_file;

    let mut file = File::open(filename).expect("Where the file at broski");
    let mut wav_file: WavFile = WavFile::default();
    unsafe {
        let mut buffer: &mut [u8] = std::slice::from_raw_parts_mut(
            (&mut wav_file as *mut WavFile) as *mut u8,
            std::mem::size_of::<WavFile>(),
        );

        let mut handle = (&mut file).take(42);
        handle.read(&mut buffer).expect("Could not read bytes");
    }
    file.seek(std::io::SeekFrom::Start(42))
        .expect("Could not seek that far");
    let bytes_read = file
        .read_to_end(&mut wav_file.data)
        .expect("Could not read");
    let seconds_to_cut = args.length;
    let bytes_to_cut: usize = (seconds_to_cut * wav_file.byte_rate) as usize;
    let byte_offset: usize = (args.offset * wav_file.byte_rate) as usize;
    let mut bytes: Vec<u8> = Vec::new();
    bytes.resize(bytes_to_cut, 0);
    bytes.copy_from_slice(
        &wav_file.data
            [byte_offset..(args.offset as usize * wav_file.byte_rate as usize) + bytes_to_cut],
    );
    let new_subchunk2_size: usize = bytes_read;
    let new_chunk_size: u32 = (36 + new_subchunk2_size) as u32;
    let new_wav_file: WavFile = WavFile {
        chunkid: wav_file.chunkid,
        chunksize: new_chunk_size,
        format: wav_file.format,
        subchunk1id: wav_file.subchunk1id,
        subchunk1size: wav_file.subchunk1size,
        audio_format: wav_file.audio_format,
        num_channels: wav_file.num_channels,
        sample_rate: wav_file.sample_rate,
        byte_rate: wav_file.byte_rate,
        block_alignment: wav_file.block_alignment,
        bits_per_sample: wav_file.bits_per_sample,
        subchunk2id: wav_file.subchunk2id,
        subchunk2size: new_subchunk2_size as u32,
        data: bytes,
    };
    unsafe {
        let struct_to_write = struct_as_u8_slice(&new_wav_file);
        let mut new_file = std::fs::File::create(args.output_file).expect("Could not create file");
        new_file.write(struct_to_write).expect("Could not write?");
        new_file
            .write(&new_wav_file.data[0..bytes_to_cut])
            .expect("Idk lmao");
    }
}
