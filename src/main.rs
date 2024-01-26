use clap::Parser;

/*

def main():
    parser = argparse.ArgumentParser(description="Extract dtbs from kernel images.")
    parser.add_argument("filename", help="Android kernel image")
    parser.add_argument(
        "-o", dest="output_dir", default="dtb", required=False, help="Output directory"
    )
    parser.add_argument(
        "-n",
        dest="extract",
        action="store_false",
        default=True,
        required=False,
        help="Do not extract, just output information",
    )
    parser.add_argument("-V", "--version", action="version", version=__version__)

    args = parser.parse_args() */

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename
    #[arg(short, long)]
    filename: String,
    #[arg(short)]
    o: bool,
    #[arg(short)]
    n: bool,
    #[arg(short, long)]
    dest: String
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();
    let filename = args.filename;
    let o = args.o;
    let n = args.n;
    let dest = args.dest;



    println!("args {} {} {} {}", filename, o, n, dest);
}
