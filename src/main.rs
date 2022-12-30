use ndarray::Array2;


fn main() {
    let mut a1 = Array2::<f64>::ones((1,3));
    let mut a2 = Array2::<f64>::ones((3,1));

    a1[[0,0]] = 3.0;

    println!("a1 is {a1}");
    println!("a2 is {a2}");

    let rmatmult = a1.dot(&a2);
    let rsum = &a1+&a1;

    println!("rsum is {rsum}");
    println!("rmatmult is {rmatmult}");
}
