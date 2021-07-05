#[derive(Debug)]
pub struct MetaData {
	pub n_dims: usize,
	pub dim_size: (usize, usize, usize),
	pub element_spacing: (f32, f32, f32),
}

impl MetaData{
	// Another static method, taking two arguments:
    pub fn parse(file_path: String) -> std::io::Result<MetaData> {
        
    	let _m_data = MetaData{
	        n_dims: 3,
	        dim_size: (512 , 512, 33),
	        element_spacing: (0.402344, 0.402344, 0.899994)
	    };

	    Ok(_m_data)
    }


    
}