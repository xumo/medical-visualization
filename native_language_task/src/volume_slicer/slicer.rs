use crate::volume_slicer::meta_data::MetaData;


pub enum Axis{
	X,
	Y,
	Z
}

pub struct VolumeSlicer{
	//meta_data: MetaData,
	width: usize,
	height: usize,
	depth: usize,
}

impl VolumeSlicer{

	pub fn new(meta_data:MetaData) -> VolumeSlicer{
		 VolumeSlicer{
        	//meta_data: meta_data,
        	width: meta_data.dim_size.0,
			height: meta_data.dim_size.1,
			depth: meta_data.dim_size.2,
    	}
	}

	pub fn slice_middle(&self, volume: &Vec<u8>, axis: Axis) -> Vec<u8>{
		match axis {
			Axis::X => self.slice_x(volume),
			Axis::Y => self.slice_y(volume),
			Axis::Z => self.slice_z(volume),			
		}		
	}

	
	fn get_index(&self, x: usize, y: usize, z: usize) -> usize {
		x + self.width
		* ( y + self.height * z )
	}

	fn from_le_and_scale(left: u8, right: u8) -> u8{
		 (right << 4 )| (left >> 4)

	}

	fn slice_x(&self,volume: &Vec<u8>) -> Vec<u8>{
		let z = self.width / 2 ;
		let mut img_data = vec![0u8; self.width * self.depth];
		for x in 0..self.width {
			for y in 0..self.depth {
				let index =  self.get_index(x,z,y);
				img_data[x + self.width * y ] = VolumeSlicer::from_le_and_scale( volume[2 * index],  volume[2 * index + 1]);
			}
		}

		img_data
	}

	fn slice_y(&self,volume: &Vec<u8>) -> Vec<u8> {

		let z = self.height / 2 ;
		let mut img_data = vec![0u8; self.height * self.depth];
		for x in 0..self.height {
			for y in 0..self.depth {
				let index =  self.get_index(z,x,y);
				img_data[x + self.height * y ] = VolumeSlicer::from_le_and_scale( volume[2 * index],  volume[2 * index + 1]);
			}
		}

		img_data
	}

	fn slice_z(&self,volume: &Vec<u8>) -> Vec<u8> {
		let z = self.depth / 2 ;
		let mut img_data = vec![0u8; self.width * self.height];
		for x in 0..self.width {
			for y in 0..self.height {
				let index =  self.get_index(x,y,z);
				img_data[x + self.width * y ] =VolumeSlicer::from_le_and_scale( volume[2 * index],  volume[2 * index + 1]);
			}
		}

		img_data
	}

}