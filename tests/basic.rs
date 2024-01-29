extern crate basilisk;

use basilisk_linalg::basmatrix;

#[test]
fn basmat_new() {
    let mut test = basmatrix::BASMatrix::new(4, 4);
    for i in 0..4 {
		for j in 0..4 {
			test.set(i, j, 2.5+(i as f64));
		}
	}
	test.print();
	test.transpose();
	test.print();
}
