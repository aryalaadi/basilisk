extern crate basilisk;

use basilisk_linalg::bastensor;

#[test]
fn basten_new() {
    let mut test = bastensor::BASTensor::new(vec![3,3,3]);
    test.set(&[0,1,2], 10.0);
    println!("{:?}", test.data);
}
