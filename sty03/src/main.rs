fn main() {
    println!("变量模块");
    let x = "我是一个不可变的变量";
    // x = "这样使用将会报错";
    let mut y = "我是一个可变的变量";
    y = "我被改变了";

    println!("常量模块");
    // 设置一个常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("使用阴影-Shadowing ");
    let A = 5; //
    let A = A + 6; // 阴影覆盖，直到它本身被遮蔽或作用域结束。
    println!("{A}"); // 11
    {
        let A = A * 2;
        println!("The value of x in the inner scope is: {A}"); // 22
    }
    println!("The value of x is: {A}"); // 11
                                        // Shadowing 对比 mut ？？？

    let spaces = "   ";
    let spaces = spaces.len();

    // 报错 不允许改变数据类型
    // let mut spacess = "   ";  
    // spacess = spacess.len();
}
