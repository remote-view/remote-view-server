use clap::Parser;

mod encode;

use encode::RawBuffer;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP to connect to
    #[arg(short, long)]
    target: String,

    /// Port to connect to
    #[arg(short, long)]
    port: u8

}

fn main() {
    let rb = RawBuffer::new();
    println!("{:?}", rb.content);
}
