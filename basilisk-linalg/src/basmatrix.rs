/*
    file: basilisk-linalg/src/basmatrix.rs
    license: GPL3
*/

pub enum BASMatrixDevice {
    OPENCL,
    CPU,
}

/*
    This static variable will determine if the BASMatrix computations
    will be done on the CPU or on the GPU with OpenCL

    BASMatrix by will compute everything on the CPU unless dev is mutated by
    basilisk_linalg::setBasiliskDevice(OPENCL)

    BY THE WAY, it is SAFE, dw
*/
static mut DEV: BASMatrixDevice = BASMatrixDevice::CPU;
pub fn set_device(d: BASMatrixDevice) {
    unsafe { DEV = d };
}

#[derive(Clone)]
pub struct BASMatrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl BASMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        BASMatrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    pub fn print(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{} ", self.get(i, j));
            }
            println!();
        }
    }

    pub fn add(&mut self, to_add: &BASMatrix) -> Result<i32, &str> {
        if self.rows == to_add.rows && self.cols == to_add.cols {
            // NOTE: it is VERY safe :p
            unsafe {
                match DEV {
                    BASMatrixDevice::CPU => self._cpu_add(to_add),
                    BASMatrixDevice::OPENCL => self._opencl_add(to_add),
                }
            }
            Ok(0)
        } else {
            Err("BASMatrix: cannot add")
        }
    }

    // I dont know if this is too slow, but it gets the job done for now
    pub fn transpose(&mut self) {
        let tmp = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i * self.cols + j] = tmp.data[j * self.cols + i];
            }
        }
    }

    /*
        MatA.BASfloatOP(); operates y=BASfloatOP(x) on every element
        BASfloatOP is a pointer a pure float function like sin(x)
        But this method is on hold until function pointers becomes stable rust
    */
    pub fn BASflatOP(&mut self, func: fn(f64) -> f64) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i * self.cols + j] = func(self.data[i * self.cols + j]);
            }
        }
    }

    fn _cpu_add(&mut self, to_add: &BASMatrix) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i * self.cols + j] =
                    self.data[i * self.cols + j] + to_add.data[i * self.cols + j];
            }
        }
    }

    /*
       turns out clc doesnt have a driver for my GPU
       and I cant run opencl through my CPU because
       my distribution has not packaged the intel
       programs and libraries for that :(
       opencl backend on hold until either I package the
       intel programs or I write the GPU driver.
     */
    fn _opencl_add(&mut self, to_add: &BASMatrix) {
        print!("TODO");
    }
}
