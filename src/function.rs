pub mod test_function {
    //返回值为数组
    pub fn add(i: i32, j: i32) -> i32 {
        i + j
    }
    //隐式返回（）
    fn println_data() {
        println!("{}", 123)
    }
    //当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )
    fn dead_end() -> ! {
        panic!("你已经到了穷途末路，崩溃吧！");
    }
}
