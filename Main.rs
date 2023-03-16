use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use clap::{App, Arg};
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn main() {
    let matches = App::new("Basic File Encryption Utility")
        .version("0.1")
        .author("Your Name <your.email@example.com>")
        .about("Encrypts and decrypts files using a symmetric-key algorithm")
        .arg(
            Arg::with_name("encrypt")
                .short("e")
                .long("encrypt")
                .value_name("FILE")
                .help("Encrypts a file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("decrypt")
                .short("d")
                .long("decrypt")
                .value_name("FILE")
                .help("Decrypts a file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Specifies the output file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .value_name("KEY")
                .help("
