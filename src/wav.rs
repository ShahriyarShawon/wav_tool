use std::fs::File;
use std::io::prelude::*;

unsafe fn struct_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    return std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>());
}
#[derive(Debug, Default)]
#[repr(C)]
pub struct WavFile {
    pub chunkid: [u8; 4],
    pub chunksize: u32,
    pub format: [u8; 4],
    pub subchunk1id: [u8; 4],
    pub subchunk1size: u32,
    pub audio_format: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_alignment: u16,
    pub bits_per_sample: u16,
    pub subchunk2id: [u8; 4],
    pub subchunk2size: u32,
    pub data: Vec<u8>,
}

impl WavFile {
    #[allow(dead_code)]
    pub fn print_wav_file(&self) -> () {
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

    pub fn from_file(file: &mut File) -> WavFile {
        let mut wav_file = WavFile::default();
        unsafe {
            let mut buffer: &mut [u8] = std::slice::from_raw_parts_mut(
                (&mut wav_file as *mut WavFile) as *mut u8,
                std::mem::size_of::<WavFile>(),
            );

            let mut handle = (file).take(42);
            handle.read(&mut buffer).expect("Could not read bytes");
        }
        wav_file
    }

    pub fn get_slice_as_wav(&mut self, file: &mut File, length: u32, offset: u32) -> WavFile {
        let bytes_read = file.read_to_end(&mut self.data).expect("Could not read");
        let seconds_to_cut = length;
        let bytes_to_cut: usize = (seconds_to_cut * self.byte_rate) as usize;
        let byte_offset: usize = (offset * self.byte_rate) as usize;
        let mut bytes: Vec<u8> = Vec::new();
        bytes.resize(bytes_to_cut, 0);
        bytes.copy_from_slice(
            &self.data[byte_offset..(offset as usize * self.byte_rate as usize) + bytes_to_cut],
        );
        let new_subchunk2_size: usize = bytes_read;
        let new_chunk_size: u32 = (36 + new_subchunk2_size) as u32;
        let new_wav_file: WavFile = WavFile {
            chunkid: self.chunkid,
            chunksize: new_chunk_size,
            format: self.format,
            subchunk1id: self.subchunk1id,
            subchunk1size: self.subchunk1size,
            audio_format: self.audio_format,
            num_channels: self.num_channels,
            sample_rate: self.sample_rate,
            byte_rate: self.byte_rate,
            block_alignment: self.block_alignment,
            bits_per_sample: self.bits_per_sample,
            subchunk2id: self.subchunk2id,
            subchunk2size: new_subchunk2_size as u32,
            data: bytes,
        };
        new_wav_file
    }

    pub fn write_wav_to_file(&self, filename: &String) {
        unsafe {
            let struct_to_write = struct_as_u8_slice(self);
            let mut new_file = std::fs::File::create(filename).expect("Could not create file");
            new_file.write(struct_to_write).expect("Could not write?");
            new_file
                .write(&self.data)
                .expect("Idk lmao");
        }
    }
}
