#[derive(Debug)]
pub struct MetaData {
	pub n_dims: usize,
	pub dim_size: (usize, usize, usize),
	pub element_spacing: (f32, f32, f32),
}

impl MetaData{
	// TODO actually parse a file
    pub fn parse(_file_path: String) -> std::io::Result<MetaData> {
        
    	let _m_data = MetaData{
	        n_dims: 3,
	        dim_size: (512 , 512, 333),
	        element_spacing: (0.402344, 0.402344, 0.899994)
	    };

	    Ok(_m_data)
    }

    pub fn width(&self) -> usize{
    	self.dim_size.0
    }

    pub fn height(&self) -> usize{
    	self.dim_size.1
    }

    pub fn depth(&self) -> usize{
    	self.dim_size.2
    }

}