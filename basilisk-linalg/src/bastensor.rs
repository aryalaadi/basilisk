/*
    file: basilisk-linalg/src/bastensor.rs
    license: GPL3
*/

#[derive(Clone)]
pub struct BASTensor {
    dim: Vec<usize>,
    data: Vec<f64>,
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
        if pos.len() != self.dim.len() {
            return Err("BASTensor: cannot set");
        }
        let mut index = 1;
        for i in pos.iter() {
            index *= i;
        }
        if index != self.size {
            return Err("BASTensor: cannot set");
        }
        self.data[index] = value;
        Ok(0)
    }
}
