use std::os::unix::ucred::peer_cred;

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
    let pred = d[0].clone();
    _pred(pred, m, 1);

    let _ = cost.add(&pred);
    let _ = cost.sub(&d[1]);
    let _ = cost.mul(&cost.clone());
    let n = cost.rows as f64;
    let _ = cost.scalardiv(n);

    return cost;
}

// c current layer
fn _pred(mut m :BASMatrix, model: &BASModelSEQ, c: usize) -> usize {
    if model.n>=c {
        let _ = m.mul(&model.layers[c].weights);
        for i in 0..m.rows*m.cols {
            m.data[i] = model.layers[c].act.activate(null, m.data[i]);
        }
        if model.n==c {return 0;}
        else { return _pred(m, model,model.n+1); }
    }
    else {
        return 0;
    }
}

impl BASCost {
    pub fn loss(self, m: &BASModelSEQ, d: &[BASMatrix; 2]) -> BASMatrix {
        match self {
            BASCost::MSE => return mse(m, d),
            BASCost::SoftMax => return todo!(),
        }
    }
}
