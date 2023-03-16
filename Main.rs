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
                .help("Specifies the 32-byte key (in Base64 format) to use for encryption/decryption")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let input_file = matches.value_of("encrypt").or_else(|| matches.value_of("decrypt")).unwrap();
    let output_file = matches.value_of("output").unwrap();
    let key_base64 = matches.value_of("key").unwrap();

    let key = base64::decode(key_base64).expect("Invalid key format");
    if key.len() != 32 {
        panic!("Key must be exactly 32 bytes long");
    }

    if let Some(encrypt_file) = matches.value_of("encrypt") {
        encrypt_file(&encrypt_file, &output_file, &key);
    } else if let Some(decrypt_file) = matches.value_of("decrypt") {
        decrypt_file(&decrypt_file, &output_file, &key);
    }
}

fn encrypt_file(input_file: &str, output_file: &str, key: &[u8]) {
    let mut file = File::open(input_file).expect("Unable to open input file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Unable to read input file");

    let iv = rand::thread_rng().gen::<[u8; 16]>();
    let cipher = Aes256Cbc::new_from_slices(key, &iv).unwrap();
    let encrypted_data = cipher.encrypt_vec(&data);

    let mut file = File::create(output_file).expect("Unable to create output file");
    file.write_all(&iv).expect("Unable to write IV to output file");
   
