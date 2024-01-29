/*
    file: basilisk-linalg/src/bastensor.rs
    license: GPL3
*/

#[derive(Clone)]
pub struct BASTensor {
    dim: Vec<usize>,
    data: Vec<f64>,
}

impl BASTensor {
    pub fn new(dim: Vec<usize>) -> Self {
		let mut size = 1;
		for i in dim.iter() {
			size *= i;
		}
		let data = vec![0.0; size];
        BASTensor {
			dim,
			data
        }
    }
}
