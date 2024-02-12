extern crate basilisk;
use basilisk::{basactivation::BASActivation, baslayer::BASLayer, basmodel::*, basoptimizer::{BASOptimizer, BASOptimizer_t}};
use basilisk_linalg::basmatrix::BASMatrix;

#[test]
fn seq_sgd_mse() {
    let optimzer = BASOptimizer::init(BASOptimizer_t::SGD, 0.00005 as f64);
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
    for _i in 0..1000 {
        optimzer.to_owned().optimize(&mut model, 
            &d, 
            1, 
            0.005);
    }
    model.layers[1].weights.print();
}

fn null(x: f64) -> f64 {
    return x;
}
#[test]
fn prediction() {
    let mut pred = BASMatrix::new(1, 1);
    pred.data[0] = 2.0;
    let mut w = BASMatrix::new(1, 1);
    w.data[0] = 1.89;

    print!("before pred: "); pred.print();
    let mut c = 0;
    loop {
        if 1>c {
            let _ = pred.mul(&w);
            for i in 0..pred.rows*pred.cols {
                pred.data[i] = BASActivation::NONE.activate(null, pred.data[i]);
                print!("once\n");
            } 
            c+=1; 
        }
        else {
            break;
        }
    }
    print!("after pred: "); pred.print();
}