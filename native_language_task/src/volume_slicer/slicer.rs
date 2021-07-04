use crate::volume_slicer::meta_data::MetaData;

pub struct VolumeSlicer{
	meta_data: MetaData
}
impl VolumeSlicer{

	pub fn new(meta_data:MetaData) -> VolumeSlicer{
		 VolumeSlicer{
        	meta_data: meta_data
    	}
	}



	pub fn slice(&self, volume: &Vec<u8>){

	}

}