use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP to connect to
    #[arg(short, long)]
    target: String,

    /// Port to connect to
    #[arg(short, long)]
    port: u16,

}

fn main() {

    let args = Args::parse();
    println!("{}", args.target);
}
