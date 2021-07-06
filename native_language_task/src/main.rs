extern crate clap;
extern crate image;
use clap::{Arg, App};

mod volume_slicer;
use volume_slicer::meta_data::MetaData;
use volume_slicer::raw_parser::RawData;
use volume_slicer::slicer::{VolumeSlicer, Axis};

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
              	   .short("r")
                   .help("Sets the raw raw file to use")
                   //.required(true)
                   .takes_value(true)
                   )
              .get_matches();


	let meta_path = matches.value_of("meta").unwrap_or("assets/sinus.mhd");
	let raw_path = matches.value_of("raw").unwrap_or("assets/sinus.raw");
    
    println!("Using files\n----------- \nmeta data: {} \nraw volume: {}", meta_path, raw_path);
    
    let meta_data = MetaData::parse(meta_path.to_string())?;
    let raw_data = RawData::parse(raw_path.to_string())?;
    
    let slicer = VolumeSlicer::new(meta_data);


    let image_buffer_x = slicer.slice_middle(raw_data.get_data(), Axis::X);
    let image_buffer_y = slicer.slice_middle(raw_data.get_data(), Axis::Y);
    let image_buffer_z = slicer.slice_middle(raw_data.get_data(), Axis::Z);

    image::save_buffer("images/slice_x.png", &image_buffer_x, 512, 333, image::ColorType::L8).unwrap();
    image::save_buffer("images/slice_y.png", &image_buffer_y, 512, 333, image::ColorType::L8).unwrap();
    image::save_buffer("images/slice_z.png", &image_buffer_z, 512, 512, image::ColorType::L8).unwrap();


	Ok(())

}
