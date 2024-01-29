/*
    file: basilisk-linalg/src/bastensor.rs
    license: GPL3
*/

#[derive(Clone)]
pub struct BASTensor {
    dim: Vec<usize>,
    pub data: Vec<f64>,
    size: usize,
}

impl BASTensor {
    pub fn new(dim: Vec<usize>) -> Self {
        let mut size = 1;
        for i in dim.iter() {
            size *= i;
        }
        let data = vec![0.0; size];
        BASTensor { dim, data, size }
    }

    pub fn set(&mut self, pos: &[usize], value: f64) -> Result<i32, &str> {
		let d = pos.len();
        if d != self.dim.len() {
            return Err("BASTensor: cannot set");
        }
        let mut flat_index = 0;
        for (i, &index) in pos.iter().enumerate() {
            if index >= self.dim[i] {
                return  Err("BASTensor: cannot set");
            }
            flat_index = flat_index * self.dim[i] + index;
        }
        self.data[flat_index] = value;
        Ok(0)
    }
}
