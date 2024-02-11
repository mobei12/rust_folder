
pub mod test {
    pub fn test_add(i: i32, j: i32) -> i32 {
        // 返回相加值，这里可以省略return
        i + j
    }
    pub fn test_float() {
        //32 浮点和64浮点的精度是不同的，末日64
        let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
        let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

        println!("abc (f32)");
        println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits()); //0.1 + 0.2: 3e99999a
        println!("         0.3: {:x}", (abc.2).to_bits()); //0.3: 3e99999a
        println!();

        println!("xyz (f64)");
        println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits()); //0.1 + 0.2: 3fd3333333333334
        println!("         0.3: {:x}", (xyz.2).to_bits()); //0.3: 3fd3333333333333
        println!();

        assert!(abc.0 + abc.1 == abc.2);
        assert!(xyz.0 + xyz.1 == xyz.2);
    }
    pub fn operator_test() {
        // 编译器会进行自动推导，给予twenty i32的类型
        let twenty = 20;
        // 类型标注
        let twenty_one: i32 = 21;
        // 通过类型后缀的方式进行类型标注：22是i32类型
        let twenty_two = 22i32;

        // 只有同样类型，才能运算
        let addition = twenty + twenty_one + twenty_two;
        println!(
            "{} + {} + {} = {}",
            twenty, twenty_one, twenty_two, addition
        );

        // 对于较长的数字，可以用_进行分割，提升可读性
        let one_million: i64 = 1_000_000;
        println!("{}", one_million.pow(2));

        // 定义一个f32数组，其中42.0会自动被推导为f32类型
        let forty_twos = [42.0, 42f32, 42.0_f32];

        // 打印数组中第一个值，并控制小数位为2位
        println!("{:.2}", forty_twos[0]);
    }
    pub fn range_test() {
        //序列
        //let a = 1..3;//1、2 不包括3
        //let a = 1..=3;//1、2、3 包括3
        let a = 'a'..='c'; //a、b、c 包括c,字母也是序列
        for i in a {
            println!("{}", i)
        }
    }
    pub fn as_test() {
        //as转化
        let a: i32 = 10;
        let b: u16 = 100;
        //因为每个类型能表达的数据范围不同，如果把范围较大的类型转换成较小的类型，会造成错误，因此我们需要把范围较小的类型转换成较大的类型，来避免这些问题的发生。
        if a < b as i32 {
            println!("Ten is less than one hundred.");
        }
    }
    pub fn num_test(){

    }
}
