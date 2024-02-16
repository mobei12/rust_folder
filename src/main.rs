//mod test;
//use num::complex::Complex;
fn main() {
    //test::test::as_test()
    /*  let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im) */

    /*  let a_u16 = 1i32 - 2;
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("'a' as u16: {}", a_u16);
    let c1 = '中';//字符
    let c1 = "中";//字符串
    //该语句块是表达式的原因是：它的最后一行是表达式，返回了 x + 1 的值，注意 x + 1 不能以分号结尾，否则就会从表达式变成语句， 表达式不能包含分号
     let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);//4
    */
    assert_eq!(print_char(), ())
}
//表达式如果不返回任何值，会隐式地返回一个 ()， assert_eq!(print_char(), ())
fn print_char() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
