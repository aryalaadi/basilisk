/*
    file: basilisk-linalg/src/basmatrix.rs
    license: LGPL3
*/
use rand::Rng;

pub enum BASMatrixDevice {
    WGPU,
    CPU,
}

/*
    This static variable will determine if the BASMatrix computations
    will be done on the CPU or on the GPU with WGPU

    BASMatrix by will compute everything on the CPU unless dev is mutated by
    basilisk_linalg::setBasiliskDevice(WGPU)
*/
static mut DEV: BASMatrixDevice = BASMatrixDevice::CPU;
pub fn set_device(d: BASMatrixDevice) {
    unsafe { DEV = d };
}

#[derive(Clone)]
pub struct BASMatrix {
    pub rows: usize,
    pub cols: usize,
    // TODO: allow for all integer and float types
    pub data: Vec<f64>,
}

impl BASMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        BASMatrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn newrand(rows: usize, cols: usize) -> Self {
        // TODO: write a random number generator
        let mut rng = rand::thread_rng();
        let mut mat = BASMatrix::new(rows, cols);
        for i in 0..mat.rows {
            for j in 0..mat.cols {
                mat.data[i * mat.cols + j] = rng.gen();
            }
            println!();
        }
        return mat;
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
                    BASMatrixDevice::WGPU => self._wgpu_add(to_add),
                }
            }
            Ok(0)
        } else {
            Err("BASMatrix: cannot add")
        }
    }

    pub fn sub(&mut self, to_sub: &BASMatrix) -> Result<i32, &str> {
        if self.rows == to_sub.rows && self.cols == to_sub.cols {
            // NOTE: it is VERY safe :p
            unsafe {
                match DEV {
                    BASMatrixDevice::CPU => self._cpu_sub(to_sub),
                    BASMatrixDevice::WGPU => self._wgpu_sub(to_sub),
                }
            }
            Ok(0)
        } else {
            Err("BASMatrix: cannot add")
        }
    }

    pub fn mul(&mut self, to_mul: &BASMatrix) -> Result<i32, &str> {
        if self.cols == to_mul.rows {
            // NOTE: it is VERY safe :p
            unsafe {
                match DEV {
                    BASMatrixDevice::CPU => self._cpu_mul(to_mul),
                    BASMatrixDevice::WGPU => self._wgpu_mul(to_mul),
                }
            }
            Ok(0)
        } else {
            Err("BASMatrix: cannot mul")
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

    pub fn sum(&self) -> f64 {
        return self.data.iter().sum();
    }

    pub fn scalarmul(&mut self, x: f64) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i * self.cols + j] = self.data[i * self.cols + j] * x;
            }
        }
    }
    pub fn scalardiv(&mut self, x: f64) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i * self.cols + j] = self.data[i * self.cols + j] / x;
            }
        }
    }
    pub fn scalaradd(&mut self, x: f64) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i * self.cols + j] = self.data[i * self.cols + j] + x;
            }
        }
    }
    /*
        MatA.BASfloatOP(); operates y=BASfloatOP(x) on every element
        BASfloatOP is a pointer a pure float function like sin(x)
    */
    pub fn BASfloatOP(&mut self, func: fn(f64) -> f64) {
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

    fn _cpu_sub(&mut self, to_sub: &BASMatrix) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i * self.cols + j] =
                    self.data[i * self.cols + j] - to_sub.data[i * self.cols + j];
            }
        }
    }

    fn _cpu_mul(&mut self, to_mul: &BASMatrix) {
        let tmp = self.clone();
        self.rows = to_mul.rows;
        // I cant imagine how slow this would be for large computations...
        for i in 0..self.rows {
            for j in 0..self.cols {
                let mut cell: f64 = 0.0;
                for k in 0..self.rows {
                    cell += tmp.data[i * self.cols + k] * to_mul.data[k * self.rows + j];
                }
                self.data[i * self.cols + j] = cell;
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

    update: instead of using an already dying compute api
    i figured it would be better to use either vulkan or wgpu
    compute, and considering wgpu is much easier, I chose to use it
    instead

    TODO: create a wgpu compute wrapper library called basilisk-compute

    also the static global variable is retarded
    might consder making these function public
    or rewrite the whole CPU/GPU thing.
    */
    fn _wgpu_add(&mut self, to_add: &BASMatrix) {
        print!("TODO");
    }
    fn _wgpu_sub(&mut self, to_mul: &BASMatrix) {
        print!("TODO");
    }
    fn _wgpu_mul(&mut self, to_mul: &BASMatrix) {
        print!("TODO");
    }
}
