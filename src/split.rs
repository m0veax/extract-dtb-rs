use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::os::unix::fs::MetadataExt;

use psi_device_tree::DeviceTree as DT;

// doodfeed is not a burger
const DTB_MAGIC: u32 = 0xd00d_feed;

pub fn split(filename: &String, n: &bool,dest: &String) -> io::Result<()> {

    println!("opening file {}", filename);
    
    let mut f = File::open(filename)?;
    let step = 8;
    let size = f.metadata().unwrap().size();

    for o in (0..size).step_by(step as usize) {

        // read first bytes
        f.seek(SeekFrom::Start(o))?;
        let buf = &mut [0u8; 4];
        f.read(buf).unwrap();
        
        // is le magic?
        if u32::from_be_bytes(*buf) == DTB_MAGIC {
            // next 4 bytes plz to get size of device tree
            f.read(buf).unwrap();
            // size is not little endian
            let size = u32::from_be_bytes(*buf) as usize;
            println!("{size:#}");

            f.seek(SeekFrom::Start(o))?;
            // create vec of size filled with zerozero
            let mut buf = vec![0; size]; 
            f.read_exact(&mut buf).unwrap();

            
            let dt = DT::load(&buf).unwrap();
            
            // does it exist? if not, rule 34
            fs::create_dir_all(dest).unwrap();

            let mut filename = String::from(format!("{o:08x}"));
            filename.push_str(".dtb");

            let path = std::path::Path::new(dest).join(filename);

            println!("{path:?}");

            let mut output = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .unwrap();
            output.write_all(&buf).unwrap();
             
        }   
    }
    

    Ok(())

}