pub mod test {
    pub fn add(i: i32, j: i32) -> i32 {
        // 返回相加值，这里可以省略return
        i + j
    }
    pub fn export() {
        let (a, mut b) = (true, false);
        println!("a = {:?}, b = {:?}", a, b);
        b = true;
        println!("{}",b)
    }
}
