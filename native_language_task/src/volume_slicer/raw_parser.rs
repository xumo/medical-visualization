
use std::io::Cursor;
use std::io::prelude::*;
extern crate byteorder;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::{
    io::{Seek,SeekFrom},
    fs::File,
};

#[derive(Debug)]
pub struct RawData {
	data: Vec<u8>
}


impl RawData{

	pub fn parse(file_path: String) -> std::io::Result<RawData> {
		
		let mut raw_file = File::open(file_path)?;

	    let mut buffer = Vec::new();
	    raw_file.read_to_end(&mut buffer)?;
	    
	    let len = buffer.len(); 
		println!("Buffer length \t {} ", len);
	   
	   Ok( RawData{
	    	data: buffer
	    })
	}


	pub fn get_data(&self) -> &Vec<u8>{
		&self.data
	}
}