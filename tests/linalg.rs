extern crate basilisk;

use basilisk_linalg::basmatrix::{self, BASMatrix};

#[test]
fn basmat_new() {
    let mut test = basmatrix::BASMatrix::new(4, 4);
    let mut c = 1;
    for i in 0..4 {
        for j in 0..4 {
            test.set(i, j, c as f64);
            c += 1;
        }
    }
    test.print();
    test.transpose();
    test.print();
    test.BASfloatOP(f64::sin);
    test.print();
    let lambda = |x: f64| -> f64 { x + 5.0 };
    test.BASfloatOP(lambda);
    test.print();

    let mut test2 = basmatrix::BASMatrix::new(4, 4);
    for i in 0..4 {
        for j in 0..4 {
            test2.set(i, j, 5 as f64);
        }
    }
    test2.print();
    test2.add(&test);
    test2.print();
    test2.mul(&test2.clone());
    test2.print();
}

#[test]
fn mul1x1square() {
    let mut m1 = BASMatrix::new(1, 1);
    let mut m2 = BASMatrix::new(1, 1);
    m2.data[0] = 0.7691175521351004;
    m1.data[0] = 2.0;
    let _  = m1.mul(&m2);
    m1.print();
}