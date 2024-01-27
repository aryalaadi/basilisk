extern crate basilisk;

use basilisk_linalg::basmatrix;

#[test]
fn basmat_new() {
    let _ = basmatrix::BASMatrix::new(8, 8);
}
