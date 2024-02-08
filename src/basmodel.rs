/*
    file: src/basmodel.rs
    license: LGPL3
*/

/*

only a basic idea on how it could work

use basilisk::*;

let SEQ = BASModelSEQ::new(3);

let inputLayer = BASLayer::flat(28*28, BASActivation::NONE);
let hiddenLayer = BASLayer::dense(128, BASActivation::RELU);
let outputLayer = BASLayer::dense(10, BASActivation::NONE);

SEQ.setLayer(inputLayer, 0);
SEQ.setLayer(hiddenLayer, 1);
SEQ.setLayer(outputLayer, 2);

SEQ.compile(BASOptimizer::SGD);
SEQ.fit(ds_train, 6, ds_test);
*/

use crate::basactivation::*;
use crate::baslayer::*;

pub struct BASModelSEQ {
    n: usize,
    layers: Vec<BASLayer>,
}

impl BASModelSEQ {
    pub fn new(n: usize) -> Self {
        BASModelSEQ {
            n,
            layers: vec![BASLayer::empty(1, BASActivation::NONE); n],
        }
    }

    // only prototype, introduce lifetimes or smth idk
    pub fn setLayer(&mut self, layer: BASLayer, n: usize) {
        self.layers[n] = layer;
    }
}
