/*
    file: src/basmodel.rs
    license: LGPL3
*/

/*
use basilisk::*;

let model = BASModelSEQ::new(3, BASOptimizer::SGD);

let inputLayer = BASLayer::flat(28*28, BASActivation::NONE);
let hiddenLayer = BASLayer::dense(784, 128, BASActivation::RELU);
let outputLayer = BASLayer::dense(128, 10, BASActivation::NONE);

model.setLayer(inputLayer, 0);
model.setLayer(hiddenLayer, 1);
model.setLayer(outputLayer, 2);

model.setTrainData(ds_train);
model.setTestData(ds_test);

TODO:
model.train(6);
model.test(ds_test);
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

    pub fn set_layer(&mut self, layer: BASLayer, n: usize) {
        self.layers[n] = layer;
    }

    pub fn set_train_data(&mut self) {
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
