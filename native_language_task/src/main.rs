use crate::byteorder::ReadBytesExt;
use byteorder::LittleEndian;
use std::{
    io::{Seek,SeekFrom},
    fs::File,
};
extern crate byteorder;

extern crate clap;
use clap::{Arg, App};
use std::io::prelude::*;

use std::io::Cursor;

fn main() -> std::io::Result<()> {
  let matches = App::new("Native Language Task")
              .version("1.0")
              .author("Rodrigo Torres <studio@rodrigotorres.net>")
              .about("Slice volumen in the 3 axis")
              .arg(Arg::with_name("info")
              	   .short("i")
                   .help("Sets the info file to use")
                   //.required(true)
                   .takes_value(true)
                   )
              .arg(Arg::with_name("volume")
              	   .short("v")
                   .help("Sets the volume raw file to use")
                   //.required(true)
                   .takes_value(true)
                   )
              .get_matches();


	let info_path = matches.value_of("info").unwrap_or("assets/sinus.mhd");
	let volume_path = matches.value_of("volume").unwrap_or("assets/sinus.raw");
    
    println!("Value for info: {} volume: {}", info_path, volume_path);

	//let info_file = File::open(info_path)?;
	let mut volume_file = File::open(volume_path)?;


	let len = volume_file.seek(SeekFrom::End(0))?;
    println!("The file is currently {} bytes long", len);


    volume_file.seek( SeekFrom::Start( 0 ) )?;


     let mut buffer = Vec::new();
    // read the whole file
    volume_file.read_to_end(&mut buffer)?;
    //buf_reader.read_u16_into::<LittleEndian>(&mut buffer[..]).unwrap();
   
    let len2 = buffer.len(); 

    let mut vec_pointer = Cursor::new(buffer);



    for _index in 0..(len2 / 2 - 1) {
        vec_pointer.seek(SeekFrom::Current(2))?;
        let val = vec_pointer.read_u16::<LittleEndian>().unwrap();
        println!("value \t\t {} ", val);
        //val = val >> 4;
        //println!("recorrido \t {} ", val);
    }

	Ok(())

}
