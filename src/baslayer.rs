/*
    file: src/baslayer.rs
    license: LGPL3
*/

use crate::basactivation::*;
use basilisk_linalg::basmatrix::BASMatrix;

#[derive(Clone)]
pub enum BASLayer_t {
    EMPTY,
    FLAT,
    DENSE,
}
#[derive(Clone)]
pub struct BASLayer {
    pub act: BASActivation,
    pub n: usize,
    pub layer_t: BASLayer_t,
    pub neurons: Vec<f64>,
    pub weights: BASMatrix,
}

impl BASLayer {
    pub fn empty(x: usize, act: BASActivation) -> BASLayer {
        BASLayer {
            act: act,
            n: x,
            layer_t: BASLayer_t::EMPTY,
            neurons: vec![0.0],
            weights: BASMatrix::new(0, 0),
        }
    }
    pub fn flat(x: usize, act: BASActivation) -> BASLayer {
        BASLayer {
            act: act,
            n: x,
            layer_t: BASLayer_t::FLAT,
            neurons: vec![0.0; x],
            weights: BASMatrix::newrand(x, 1),
        }
    }
    pub fn dense(inp: usize, out: usize, act: BASActivation) -> BASLayer {
        BASLayer {
            act: act,
            n: out,
            layer_t: BASLayer_t::DENSE,
            neurons: vec![0.0; out],
            weights: BASMatrix::newrand(inp, out),
        }
    }
}
