struct BASMatrix {
	rows: usize,
    	cols: usize,
    	data: Vec<f64>,
}

impl BASMatrix {

	fn new(rows: usize, cols: usize) -> Self {
    		BASMatrix {
      			rows,
            		cols,
            		data: vec![0.0; rows * cols],
        	}
    	}

    	fn get(&self, row: usize, col: usize) -> f64 {
        	self.data[row * self.cols + col]
    	}

    	fn set(&mut self, row: usize, col: usize, value: f64) {
        	self.data[row * self.cols + col] = value;
    	}

    	fn print(&self) {
        	for i in 0..self.rows {
            		for j in 0..self.cols {
                		print!("{} ", self.get(i, j));
            		}
            	println!();
        	}
    	}
}
