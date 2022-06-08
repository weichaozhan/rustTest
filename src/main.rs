fn main() {
    // Grammar
    const A:char = 'A';

    println!("test const {0}", A);

    let a:u64 = 2;
    println!("test let {0}", a);

    let mut a:f64 = 2.3;
    println!("test let mut {0}", a);

    a = 8.8;
    println!("test let mut change {0}", a);

    // Base types
    let ia: i128 = -123456;
    let ub: u128 = 123456;

    let fc: f64 = 6.6;
    
    let bd: bool = false;

    let ce: char = 'e';

    let _tf = (1, 'a', -2.3);
    let _ag = [1, 2, 3];


    println!("ia:{0}, ub: {1}, fc: {2}, bd: {3}, ce: {4}", ia, ub, fc, bd, ce);
}
