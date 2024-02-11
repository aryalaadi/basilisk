/*
    file:       bascost.rs
    license:    LGPL3
*/
use basilisk_linalg::basmatrix::BASMatrix;

use crate::basmodel::BASModelSEQ;

pub enum BASCost {
    MSE,
    SoftMax,
}

// NOTE: this is a stupid hack
fn null(x:f64) -> f64 {
    return x;
}

fn mse(m: &BASModelSEQ, d: &[BASMatrix; 2]) -> BASMatrix {
    let mut cost = BASMatrix::new(d[1].rows, d[1].cols);
    let mut pred = BASMatrix::new(d[0].rows, d[0].cols);

    let mut c = 1;
    loop {
        if m.n>=c {
            let _ = pred.mul(&m.layers[c].weights);
            for i in 0..pred.rows*pred.cols {
                pred.data[i] = m.layers[c].act.activate(null, pred.data[i]);
            }
            if m.n==c {break}
            else { c+=1; }
        }
        else {
            break;
        }
    }

    let _ = cost.add(&pred);
    let _ = cost.sub(&d[1]);
    let _ = cost.mul(&cost.clone());
    let n = cost.rows as f64;
    let _ = cost.scalardiv(n);

    return cost;
}

impl BASCost {
    pub fn loss(self, m: &BASModelSEQ, d: &[BASMatrix; 2]) -> BASMatrix {
        match self {
            BASCost::MSE => return mse(m, d),
            BASCost::SoftMax => return todo!(),
        }
    }
}
