/*
    file: src/basoptimizer.rs
    license: LGPL3
*/

use basilisk_linalg::basmatrix::BASMatrix;

use crate::bascost::BASCost;

pub enum BASOptimizer_t {
    SGD,
}

pub struct BASOptimizer {
    t: BASOptimizer_t,
    h: BASMatrix,
}

impl BASOptimizer {
    pub fn init(t: BASOptimizer_t, h: BASMatrix) -> Self{
        BASOptimizer {
            t: t,
            h: h,
        }
    }
    pub fn optimize(self, mut w: BASMatrix, d: BASMatrix, rate: f64) {
        let mut x = w.clone();
        let _ = x.add(&self.h);
        let mut x1 = BASCost::loss(BASCost::MSE, &x, &d);
        let _ = x1.sub(&BASCost::loss(BASCost::MSE, &w, &d));

        /*
            yet another stupid hack
        */
        for i in 0..x1.rows {
            for j in 0..x1.cols {
                x1.data[i * x1.cols + j] = (x1.data[i * x1.cols + j])/self.h.data[0];
            }
        }
        for i in 0..x1.rows {
            for j in 0..x1.cols {
                x1.data[i * x1.cols + j] = (x1.data[i * x1.cols + j])*rate;
            }
        }
        let _ = w.sub(&x1);
    }
}
