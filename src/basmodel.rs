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

pub struct BASModelSEQ {
	n: i32,
	layers: Vec<BASLayer>,	
}

impl BASModelSEQ {
	pub fn new(n: i32) -> Self{
		BASModelSEQ {
			n,
			layers: vec![BASLayer::empty(0, BASActivation::NONE); n],
		}
	}

	// only prototype, introduce lifetimes or smth idk
	pub fn setLayer(&mut self, layer: BASLayer, n: i32) {
		layers[n] = layer;
	}
}
