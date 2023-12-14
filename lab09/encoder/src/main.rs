use clap::Parser;

use std::env;
#[cfg(target_os = "windows")]
const OS: &str = "Windows";
#[cfg(target_os = "linux")]
const OS: &str = "Linux";
#[cfg(target_os = "macos")]
const OS: &str = "Mac";

#[derive(Parser)]
#[command(version, about = "Encoder utility")]
struct Args {
    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    println!(
        "encoder, version {}, built for {}",
        env!("CARGO_PKG_VERSION"),
        OS
    );
    let args = Args::parse();

    if let Some(input_file) = args.input {
        let bytes = std::fs::read(&input_file).expect("Eroare la citire din input!");
        let encoded = base64::encode(&bytes);
        if let Some(output_file) = args.output {
            std::fs::write(&output_file, encoded).expect("Eroare la scriere in output");
        } else {
            println!("{}", encoded);
        }
    } else {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Eroare la citirea din stdin!");
        let encoded = base64::encode(input.trim().as_bytes());
        println!("{}", encoded);
    }
}
