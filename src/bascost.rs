/*
    file:       bascost.rs
    license:    LGPL3
*/

use basilisk_linalg::basmatrix::BASMatrix;

pub enum BASCost {
    MSE,
    SoftMax,
}

fn mse(w: &BASMatrix, d: &BASMatrix) -> BASMatrix {
    let mut tmp = w.clone();
    let _ = tmp.sub(&d);
    _ = tmp.mul(&w);

    /*
        this is such a dirty hack 
        The correct way to do this is to define a function 
        which knows how many rows the matrices have 
        and it should be a simple f(x) = x/w.rows
        and then pass the lambda function pointer to
        BASfloatOp which should do the operation for us
        BUT rust wont let me use w.rows anonymously 
        so I had to resort to basically copying a block 
        from the BASfloatOp impl
     */
    for i in 0..tmp.rows {
        for j in 0..tmp.cols {
            tmp.data[i * tmp.cols + j] = (tmp.data[i * tmp.cols + j])/tmp.rows as f64;
        }
    }

    return tmp;
}

impl BASCost {
    pub fn loss(self, w: &BASMatrix, d: &BASMatrix) -> BASMatrix {
        match self {
            BASCost::MSE => return mse(w, d),
            BASCost::SoftMax => return todo!(),
        }
    }
}
