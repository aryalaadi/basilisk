/*
    file:       src/bascost.rs
    license:    LGPL3
*/

use crate::basmodel::BASModelSEQ;
use basilisk_linalg::basmatrix::BASMatrix;

pub enum BASCost {
    MSE,
    SoftMax,
}

fn mse(m: &BASModelSEQ, d: &[BASMatrix; 2]) -> BASMatrix {
    let mut cost = BASMatrix::new(d[1].rows, d[1].cols);
    let mut pred = d[0].clone();

    let mut c = 1;
    loop {
        if m.n > c {
            match pred.mul(&m.layers[c].weights) {
                Ok(_) => {}
                Err(x) => {
                    print!("{}", x)
                }
            };
            for i in 0..pred.rows * pred.cols {
                pred.data[i] = m.layers[c].act.activate(pred.data[i]);
            }
            c += 1;
        } else {
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
