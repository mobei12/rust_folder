use function::test_function;

//mod test;
//use num::complex::Complex;
mod function;
fn main() {
    //test::test::as_test()
    /*  let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im) */

    println!("{}",test_function::add(2, 3))
}
