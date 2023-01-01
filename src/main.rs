use ndarray::Array2;
use ndarray::Array;
use ndarray::array;
use ffix::types::Ffix; 


fn main() {

    // -------------------Basic experiments with ndarrays-------------------
    let mut a1 = Array2::<f64>::ones((1,3));
    let a2 = Array2::<f64>::ones((3,1));

    a1[[0,0]] = 3.0;

    println!("a1 is {a1}");
    println!("a2 is {a2}");

    let rmatmult = a1.dot(&a2);
    let rsum = &a1+&a1;

    println!("rsum is {rsum}");
    println!("rmatmult is {rmatmult}");

    let rng = Array::range(0., 10., 1.);

    let myarr = array![[[2.,1.], [3.5,4.], [5.2,0.]]];

    println!("Array::range(0., 10., 1.0) is {rng}");
    println!("array![[[1,2], [3,4], [5,6]]] is {myarr}");

    println!("ndim of array![[[1,2], [3,4], [5,6]]]: {}", myarr.ndim());
    println!("shape of array![[[1,2], [3,4], [5,6]]]: {:?}", myarr.shape());

    println!("elements of unrolled array![[[1,2], [3,4], [5,6]]]:");
    for (idx, elem) in myarr.iter().enumerate() {
        println!("{}th element: {}", idx, elem);
    }
    // ---------------------------------------------------------------------

    // -------------------Experiments with Ffix-----------------------------
    let a: Ffix::<true, 18, 12, 'z'> = Ffix::new(10.1);
    let b: Ffix::<true, 18, 12, 'z'> = Ffix::new(1.2);

    let farr1: Array<Ffix<true, 18, 12, 'z'>, _> = array![a,b];
    let farr2: Array<Ffix<true, 18, 12, 'z'>, _> = array![a,a];

    println!("{:?}", farr1);
    println!("{:?}", farr1+farr2);
    // ---------------------------------------------------------------------

}
