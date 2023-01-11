fn main() {
    // 内存分配
    {
        let s = String::from("hello");
        println!("{}", s);
        /*
         * 我们可以将我们String需要的内存返回给分配器：当s超出范围时。
         *    当一个变量超出作用域时，Rust 会为我们调用一个特殊的函数。
         *    这个函数被称为 drop，它是作者String可以放置代码以返回内存的地方。Rustdrop在结束的大括号处自动调用。
         */
    }

    // 重新赋值 超出堆栈前置变量失效
    {
        let s1 = String::from("hello");
        let s2 = s1;
        print!("{}", s2);

        // 早些时候，我们说过，当一个变量超出范围时，Rust 会自动调用该drop函数并清理该变量的堆内存。
        // 但两个数据指针指向同一位置。这是一个问题：当s2和s1超出范围时，它们都会尝试释放相同的内存。
        // 这被称为双重释放错误，是我们之前提到的内存安全错误之一。两次释放内存会导致内存损坏，这可能会导致安全漏洞。

        //  为了确保内存安全，在行之后let s2 = s1;，Rust 认为s1不再有效。s1因此，Rust在超出作用域时不需要释放任何东西。
        // println!("{}, world!", s1); // 直接报错

        // 暗示了一个设计选择：Rust 永远不会自动创建数据的“深”副本。因此， 可以假设任何自动复制在运行时性能方面都是廉价的。

        // 如果是一开始堆栈数据是固定好的，那么前置变量并不会失效
        //   在编译时具有已知大小的整数等类型完全存储在堆栈中，因此可以快速复制实际值。
        let x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y);
        // 这里的深拷贝和浅拷贝没有区别，所以调用clone和通常的浅拷贝没有什么不同，我们可以省略它。
    }

    // 进行变量和数据克隆
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    // 权和职能分配 --函数传参
    // 将值传递给函数的机制类似于将值分配给变量时的机制。
    {
        let s = String::from("hello");
        takes_ownership(s);
        let x = 5;
        makes_copy(x);
    }

    // 返回值和范围
    // 返回值也可以转移所有权。
    {
        let s1 = gives_ownership();
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        println!("{}", s1);
        // println!("{}",s2);
        println!("{}", s3);
    }

    // 使用多个返回值
    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); 
    (s, length)
}
