/*
    file: src/baslayer.rs
    license: LGPL3
*/

use crate::basactivation::*;

#[derive(Clone)]
pub struct BASLayer {
    act: BASActivation,
    layers: usize,
}

impl BASLayer {
    pub fn empty(x: usize, act: BASActivation) -> BASLayer {
        BASLayer {
            act: act,
            layers: x,
        }
    }
}
