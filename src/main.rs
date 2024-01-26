use clap::Parser;
mod split;

use split::split;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename
    #[arg(short, long)]
    filename: String,
    #[arg(short)]
    o: String,
    #[arg(short)]
    n: bool,
    #[arg(short, long)]
    dest: String
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();
    let filename = args.filename;
    let o: String = args.o;
    let n = args.n;
    let dest = args.dest;

    println!("args {} {} {} {}", filename, o, n, dest);

    split(&filename, &n, &o, &dest);

}
