# wav_tool 
---
wav_tool is a program that takes a `.wav` file as input and 
spits out another wav file of a user specified duration at
a user specified offset (seconds).

I wrote this in an attempt to learn about media file formats (audio, video, images) while also learning
how to program in rust.

The implementation is naive, there is next to no error handling
and there's probably a lot of bad practices. I hacked this code up in a fairly short amount of time and plan on adding to it.

# Release Build 
```sh
cargo build --release
```

# Help Command 
```sh
./target/release/wav_tool --help
```

# Usage
```sh
Usage: wav_tool [OPTIONS] --input-file <INPUT_FILE> --output-file <OUTPUT_FILE> --length <LENGTH>

Options:
  -i, --input-file <INPUT_FILE>    
  -o, --output-file <OUTPUT_FILE>  
  -l, --length <LENGTH>            
      --offset <OFFSET>            [default: 0]
  -h, --help                       Print help
  -V, --version                    Print version
```

# Installation
```sh
# replace /usr/local/sbin with a location 
# anywhere else that your PATH is aware of
cp ./target/release/wav_tool  /usr/local/sbin 
```