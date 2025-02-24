use clap::Parser;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use psi_device_tree::DeviceTree as DT;
use std::os::unix::fs::MetadataExt;


// doodfeed is not a burger
const DTB_MAGIC: u32 = 0xd00d_feed;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename
    #[arg(short, long)]
    filename: String,
    #[arg(short)]
    n: bool,
    #[arg(short, long)]
    dest: String
}

fn main() {

    let args = Args::parse();
    let filename = args.filename;
    let n = args.n;
    let dest = args.dest;
    
    let mut f = File::open(filename).unwrap();
    let step = 8;
    let size = f.metadata().unwrap().size();

    for o in (0..size).step_by(step as usize) {

        // read first bytes
        f.seek(SeekFrom::Start(o)).unwrap();
        let buf = &mut [0u8; 4];
        f.read(buf).unwrap();
        
        // is le magic?
        if u32::from_be_bytes(*buf) == DTB_MAGIC {
            // next 4 bytes plz to get size of device tree
            f.read(buf).unwrap();
            // size is not little endian
            let size = u32::from_be_bytes(*buf) as usize;
            println!("{size:#}");

            f.seek(SeekFrom::Start(o)).unwrap();
            // create vec of size filled with zerozero
            let mut buf = vec![0; size]; 
            f.read_exact(&mut buf).unwrap();

            
            let dt = DT::load(&buf).unwrap();
            
            // does it exist? if not, rule 34
            fs::create_dir_all(&dest).unwrap();

            let mut filename = String::from(format!("{o:08x}"));
            filename.push_str(".dtb");

            let path = std::path::Path::new(&dest).join(filename);

            println!("{path:?}");

            let mut output = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .unwrap();
            output.write_all(&buf).unwrap();
             
        }   
    }
    

}
