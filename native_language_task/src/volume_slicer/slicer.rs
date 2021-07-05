use crate::volume_slicer::meta_data::MetaData;

pub enum Axis{
	X,
	Y,
	Z
}


pub struct VolumeSlicer{
	meta_data: MetaData
}
impl VolumeSlicer{

	pub fn new(meta_data:MetaData) -> VolumeSlicer{
		 VolumeSlicer{
        	meta_data: meta_data
    	}
	}

	pub fn slice_middle(&self, volume: &Vec<u8>, axis: Axis) -> Vec<u8>{
		match axis {
			Axis::X => self.slice_x(volume),
			Axis::Y => self.slice_y(volume),
			Axis::Z => self.slice_z(volume),			
		}		
	}

	//[x + WIDTH * ( y + DEPTH * z)]
	fn get_index(&self, x: usize, y: usize, z: usize) -> usize {
		x + self.meta_data.dim_size.0 
		* ( y + self.meta_data.dim_size.1 * z )
	}

	fn slice_x(&self,volume: &Vec<u8>) -> Vec<u8>{
		println!("slice_x {}", volume.len());
		let w = self.meta_data.dim_size.0;
		let h = self.meta_data.dim_size.1;
		let d = self.meta_data.dim_size.2;
		let z = w / 2 ;
		println!("Slicing at x = {}", z);
		let mut img_data = vec![0u8; w * d];
		for x in 0..w {
			for y in 0..d {
				let index =  self.get_index(x,z,y);
				let val = volume[2 * index ];
				img_data[x + w * y ] = val.swap_bytes();
			}
		}


		img_data
	}

	fn slice_y(&self,volume: &Vec<u8>) -> Vec<u8> {
		println!("slice_x {}", volume.len());
		let w = self.meta_data.dim_size.0;
		let h = self.meta_data.dim_size.1;
		let d = self.meta_data.dim_size.2;
		let z = h / 2 ;
		println!("Slicing at x = {}", z);
		let mut img_data = vec![0u8; w * d];
		for x in 0..w {
			for y in 0..d {
				let index =  self.get_index(z,x,y);
				let val = volume[2 * index  ];
				img_data[x + w * y ] = val;
			}
		}


		img_data

	}

	fn slice_z(&self,volume: &Vec<u8>) -> Vec<u8> {
		println!("slice_z {}", volume.len());
		let w = self.meta_data.dim_size.0;
		let h = self.meta_data.dim_size.1;
		let d = self.meta_data.dim_size.2;
		let z = d / 2 ;
		println!("Slicing at z = {}", z);
		let mut img_data = vec![0u8; w * h];
		for x in 0..w {
			for y in 0..h {
				let index =  self.get_index(x,y,z);
				let val = volume[2 * index ];
				img_data[x + w * y ] = val.swap_bytes();
			}
		}


		img_data
	}

}