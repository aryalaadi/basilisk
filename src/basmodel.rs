/*
    file: src/basmodel.rs
    license: LGPL3
*/

/*

only a basic idea on how it could work

use basilisk::*;

let SEQ = BASModelSEQ::new(3, BASOptimizer::SGD);

let inputLayer = BASLayer::flat(28*28, BASActivation::NONE);
let hiddenLayer = BASLayer::dense(784, 128, BASActivation::RELU);
let outputLayer = BASLayer::dense(128, 10, BASActivation::NONE);

SEQ.setLayer(inputLayer, 0);
SEQ.setLayer(hiddenLayer, 1);
SEQ.setLayer(outputLayer, 2);

SEQ.compile();
SEQ.fit(ds_train, 6, ds_test);
*/

use crate::basactivation::*;
use crate::baslayer::*;
use crate::basoptimizer::*;

pub struct BASModelSEQ {
    n: usize,
    layers: Vec<BASLayer>,
    optimizer: BASOptimizer,
}

impl BASModelSEQ {
    pub fn new(n: usize, optmizer: BASOptimizer) -> Self {
        BASModelSEQ {
            n,
            layers: vec![BASLayer::empty(1, BASActivation::NONE); n],
            optimizer: optmizer,
        }
    }

    pub fn setLayer(&mut self, layer: BASLayer, n: usize) {
        self.layers[n] = layer;
    }

    pub fn compile(&mut self) {}
}
