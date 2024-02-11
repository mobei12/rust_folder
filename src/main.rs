mod test;
use num::complex::Complex;
// 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
fn main() {
    //test::test::as_test()
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}
