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

SEQ.setTrainData(ds_train);
SEQ.setTestData(ds_test);

SEQ.train(6);
SEQ.test(ds_test);

Once basilisk can do this much and works as expected 
the PR #6 will be merged 
*/

use crate::basactivation::*;
use crate::baslayer::*;
use crate::basoptimizer::*;

#[derive(Clone)]
pub struct BASModelSEQ {
    pub n: usize,
    pub layers: Vec<BASLayer>,
    pub optimizer: BASOptimizer,
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

    pub fn setTrainData(&mut self) {
        // large buffer of input values, a matirx of [n_ds, BASLayer.n]
        // in the model struct add new attributes traindata testdata
        // set traindata to that matrix 
        // also same for test 
    }

    pub fn train() {
        // a simple for loop n times 
        // another loop for the layers
        // update the weights and activation for each layer in seq
        // optimize 
        
        // now make sure the weights and activation can be accessed later on
    }
}
