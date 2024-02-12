/*
    file: src/basoptimizer.rs
    license: LGPL3
*/

use basilisk_linalg::basmatrix::BASMatrix;
use crate::{bascost::BASCost, basmodel::BASModelSEQ};

#[derive(Clone)]
pub enum BASOptimizer_t {
    SGD,
}

#[derive(Clone)]
pub struct BASOptimizer {
    t: BASOptimizer_t,
    h: f64,
}

impl BASOptimizer {
    pub fn init(t: BASOptimizer_t, h: f64) -> Self{
        BASOptimizer {
            t: t,
            h: h,
        }
    }

    pub fn optimize(self, model: &mut BASModelSEQ, d: &[BASMatrix; 2], layer: usize, rate: f64) {
        let mut wx = model.clone();
        wx.layers[layer].weights.scalaradd(self.h);

        let mut x1 = BASCost::loss(BASCost::MSE, &wx, &d);
        let _ = x1.sub(&BASCost::loss(BASCost::MSE, &model, &d));

        x1.scalarmul(rate/self.h);
        
        let _ = model.layers[layer].weights.sub(&x1);
    }
}
