


extern crate clap;
use clap::{Arg, App};

mod volume_slicer;
use volume_slicer::meta_data::MetaData;
use volume_slicer::raw_parser::RawData;
use volume_slicer::slicer::VolumeSlicer;

fn main() -> std::io::Result<()> {
  let matches = App::new("Native Language Task")
              .version("1.0")
              .author("Rodrigo Torres <studio@rodrigotorres.net>")
              .about("Slice rawn in the 3 axis")
              .arg(Arg::with_name("meta")
              	   .short("i")
                   .help("Sets the meta file to use")
                   //.required(true)
                   .takes_value(true)
                   )
              .arg(Arg::with_name("raw")
              	   .short("v")
                   .help("Sets the raw raw file to use")
                   //.required(true)
                   .takes_value(true)
                   )
              .get_matches();


	let meta_path = matches.value_of("meta").unwrap_or("assets/sinus.mhd");
	let raw_path = matches.value_of("raw").unwrap_or("assets/sinus.raw");
    
    println!("Value for meta: {} raw: {}", meta_path, raw_path);

    
    let m_data = MetaData{
        n_dims: 3,
        dim_size: (512 , 512, 33),
        element_spacing: (0.402344, 0.402344, 0.899994)
    };

    let raw_data = RawData::parse(raw_path.to_string())?;
    
    let slicer = VolumeSlicer::new(m_data);


    slicer.slice(raw_data.get_data());



	Ok(())

}
