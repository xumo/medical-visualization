use crate::volume_slicer::meta_data::MetaData;


pub enum Axis{
	X,
	Y,
	Z
}


fn from_le_and_scale(left: u8, right: u8) -> u8{
	// Only 12 bits are used and they are in little endian 
	// Shift the byte on the right 4  bits to the left as the first 4 are not used
	// and because is little endian place it at the beginning
	// The put the first 4 bit from  the other byte to the right
	// this loses 4 bits of information as expected 
	(right << 4 ) | (left >> 4)
}

pub struct VolumeSlicer{
	width: usize,
	height: usize,
	depth: usize,
}

impl VolumeSlicer{

	pub fn new(meta_data:MetaData) -> VolumeSlicer{
		 VolumeSlicer{
        	width: meta_data.width(),
			height: meta_data.height(),
			depth: meta_data.depth(),
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
		x + self.width * ( y + self.height * z )
	}

	
	fn slice_x(&self,volume: &Vec<u8>) -> Vec<u8>{
		let z = self.width / 2 ;
		let mut img_data = vec![0u8; self.width * self.depth];
		for x in 0..self.width {
			for y in 0..self.depth {
				let index =  self.get_index(x,z,y);
				img_data[x + self.width * y ] = from_le_and_scale( volume[2 * index],  volume[2 * index + 1]);
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
				img_data[x + self.height * y ] = from_le_and_scale( volume[2 * index],  volume[2 * index + 1]);
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
				img_data[x + self.width * y ] = from_le_and_scale( volume[2 * index],  volume[2 * index + 1]);
			}
		}

		img_data
	}

}


#[test]
fn test_from_le_and_scale() {
	// Calculating for Max count 
	// 4095 -> 255
	// 0x0FFF (BE) or 0xFF0F(LE) -> 0xFF

	let mut left = 255u8;
	let mut right = 127u8;
	
	let mut result = from_le_and_scale(left, right);
	assert_eq!(result ,0xFF );
 
 	left = 0xFFu8;
 	right = 0x0Fu8;

	result = from_le_and_scale(left, right);
	assert_eq!(result ,0xFF );

	// Calculating for zero 
	// 0 -> 0
	// 0x0000 (BE) or 0x0000(LE) -> 0x00
	left = 0x00u8;
 	right = 0x00u8;

	result = from_le_and_scale(left, right);
	assert_eq!(result ,0x00 );

	// Calculating for half 
	// 2046 -> 127
	// 0x07FF (BE) or 0xFF07(LE) -> 0x7F
	left = 0xFFu8;
 	right = 0x07u8;

	result = from_le_and_scale(left, right);
	assert_eq!(result ,0x7F );


}

