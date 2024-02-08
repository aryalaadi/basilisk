/*
    file:       bascost.rs
    license:    LGPL3
*/

use basilisk_linalg::basmatrix::BASMatrix;

pub enum BASCost {
    MSE,
    SoftMax,
}

fn mse(mut w: BASMatrix, d: BASMatrix) -> f64 {
    let mut cost: f64 = 0.0;
    let _ = w.sub(&d);
    _ = w.mul(&w.clone());
    cost = w.sum() / (w.rows as f64);
    return cost;
}

impl BASCost {
    pub fn loss(self, w: BASMatrix, d: BASMatrix) -> f64 {
        match self {
            BASCost::MSE => return mse(w, d),
            BASCost::SoftMax => return 0.0,
        }
    }
}
