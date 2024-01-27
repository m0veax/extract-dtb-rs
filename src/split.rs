use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::os::unix::fs::MetadataExt;

use psi_device_tree::DeviceTree as DT;

const DTB_MAGIC: u32 = 0xd00d_feed;

pub fn split(filename: &String, n: &bool, o: &String, dest: &String) -> io::Result<()> {

    println!("opening file {}", filename);
    
    let mut f = File::open(filename)?;
    let step = 8;
    let size = f.metadata().unwrap().size();

    for o in (0..size).step_by(step as usize) {

        //println!("Offset {}", o);
        f.seek(SeekFrom::Start(o))?;
        let buf = &mut [0u8; 4];
        let val = f.read(buf).unwrap() as u32;

        let hex = u32::from_be_bytes(*buf);
        
        if hex == DTB_MAGIC {
            f.seek(SeekFrom::Start(o))?;

            println!("{hex:#08x?} @{o:#x}"); // I like

            let mut buf = Vec::new();
            f.read_to_end(&mut buf).unwrap();

            let dt = DT::load(&buf);
            //println!("{dt:#?} {:?}", buf[0]);

            /* 
            let dtb = dt.store().unwrap();

           
            let mut output = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(dest + o.to_string().to_owned() + ".dtb")
                .unwrap();
            output.write_all(&dtb).unwrap();
            */
        }   
    }
    

    Ok(())

}