use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex::{decode, encode};
use std::{env, fs};
use structopt::StructOpt;

// Alias for the AES256-CBC mode with PKCS7 padding
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

#[derive(StructOpt, Debug)]
#[structopt(name = "Basic File Encryption Utility")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Encrypt {
        #[structopt(short, long)]
        input: String,

        #[structopt(short, long)]
        output: String,

        #[structopt(short, long)]
        key: String,
    },
    Decrypt {
        #[structopt(short, long)]
        input: String,

        #[structopt(short, long)]
        output: String,

        #[structopt(short, long)]
        key: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    match opt.cmd {
        Command::Encrypt { input, output, key } => {
            let data = fs::read(input)?;
            let encrypted_data = encrypt(&data, &key)?;
            fs::write(output, encrypted_data)?;
        }
        Command::Decrypt { input, output, key } => {
            let data = fs::read(input)?;
            let decrypted_data = decrypt(&data, &key)?;
            fs::write(output, decrypted_data)?;
        }
    }

    Ok(())
}

fn encrypt(data: &[u8], key: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let key = decode(key)?;
    let iv = generate_iv();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv)?;

    let encrypted_data = cipher.encrypt_vec(data);
    let mut result = iv.to_vec();
    result.extend_from_slice(&encrypted_data);

    Ok(result)
}

fn decrypt(data: &[u8], key: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let key = decode(key)?;
    let (iv, encrypted_data) = data.split_at(16);
    let cipher = Aes256Cbc::new_from_slices(&key, iv)?;

    let decrypted_data = cipher.decrypt_vec(encrypted_data)?;

    Ok(decrypted_data)
}

fn generate_iv() -> [u8; 16] {
    let mut iv = [0u8; 16];
    let rng = rand::thread_rng();
    for i in 0..16 {
        iv[i] = rng.gen();
    }
    iv
}
