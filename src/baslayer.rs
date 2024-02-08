/*
    file: src/baslayer.rs
    license: LGPL3
*/

use crate::basactivation::*;

#[derive(Clone)]
pub enum BASLayer_t {
    EMPTY,
    FLAT,
    DENSE,
}
#[derive(Clone)]
pub struct BASLayer {
    act: BASActivation,
    layers: usize,
    layer_t: BASLayer_t,
}

impl BASLayer {
    pub fn empty(x: usize, act: BASActivation) -> BASLayer {
        BASLayer {
            act: act,
            layers: x,
            layer_t: BASLayer_t::EMPTY,
        }
    }
    pub fn flat(x: usize, act: BASActivation) -> BASLayer {
        BASLayer {
            act: act,
            layers: x,
            layer_t: BASLayer_t::FLAT,
        }
    }
    pub fn dense(x: usize, act: BASActivation) -> BASLayer {
        BASLayer {
            act: act,
            layers: x,
            layer_t: BASLayer_t::DENSE,
        }
    }
}
