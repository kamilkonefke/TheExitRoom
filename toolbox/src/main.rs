use clap::Parser;
use base64::prelude::*;
use core::{panic, str};

#[derive(Parser, Debug)]
#[command(name = "toolbox")]
#[command(version = "0.3")]
#[command(about = "I am a program and I work.", long_about = None)]
struct Cli {
    #[arg(short, long, action)]
    decrypt: bool,
    #[arg(short, long, action)]
    encrypt: bool,
    #[arg(short, long, value_name = "MESSAGE")]
    message: Option<String>,
    //#[arg(short, long, value_name = "PATH")]
    //image: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // Encrypt
    if cli.encrypt == true {
        if cli.message.is_some() {
            match cli.message {
                Some(ref m) => println!("{}", message_encrypt(&m)),
                _ => panic!("No input"),
            }
        }

    /*
        if cli.image.is_some() {
            match cli.image {
                Some(ref path) => image_encrypt(&path),
                _ => panic!("No input"),
            }
        }
    */
    }

    // Decrypt
    if cli.decrypt == true {
        if cli.message.is_some() {
            match cli.message {
                Some(ref m) => println!("{}", message_decrypt(&m)),
                _ => panic!("No input"),
            }
        }

    /*
        if cli.image.is_some() {
            match cli.image {
                Some(ref path) => image_decrypt(&path),
                _ => panic!("No input"),
            }
        } 
    */
    }
}

fn message_encrypt(message: &str) -> String {
    BASE64_STANDARD_NO_PAD.encode(message)
}

fn message_decrypt(message: &str) -> String {
    let m = BASE64_STANDARD_NO_PAD.decode(message).unwrap();
    match str::from_utf8(&m) {
        Ok(o) => return o.to_string(),
        Err(_) => panic!("Err"),
    }
}

fn image_encrypt(path: &str) {

}

fn image_decrypt(path: &str) {

}
