extern crate basilisk;
use basilisk::{baslayer::BASLayer, basmodel::*, basoptimizer::{BASOptimizer, BASOptimizer_t}};
use basilisk_linalg::basmatrix::BASMatrix;

#[test]
fn seq_sgd_mse() {
    let optimzer = BASOptimizer::init(BASOptimizer_t::SGD, 0.00005);
    let mut model = BASModelSEQ::new(2, optimzer.clone());

    let first_layer = BASLayer::flat(1, basilisk::basactivation::BASActivation::NONE);
    let output_layer = BASLayer::dense(1, 1, basilisk::basactivation::BASActivation::NONE);

    model.set_layer(first_layer, 0);
    model.set_layer(output_layer, 1);

    let mut input = BASMatrix::new(1, 1);
    let mut output = BASMatrix::new(1, 1);
    input.data[0] = 2.0;
    output.data[0] = 4.0;

    let d = [input, output];
    model.layers[1].weights.print();
    for _i in 0..5 {
        optimzer.to_owned().optimize(model.to_owned(), 
            &d, 
            1, 
            0.0005);
    }
    model.layers[1].weights.print();
}
