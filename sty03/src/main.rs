fn main() {
    // 变量模块
    {
        // 变量的使用
        {
            let x = "我是一个不可变的变量";
            // x = "这样使用将会报错";
            let mut y = "我是一个可变的变量";
            y = "我被改变了";
        }

        // 常量的使用
        {
            // 设置一个常量
            const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        }

        // 使用阴影-Shadowing
        {
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
    }

    // 数据类型
    {
        // 整数类型
        // Length	Signed	Unsigned
        // 8-bit	i8	u8
        // 16-bit	i16	u16
        // 32-bit	i32	u32
        // 64-bit	i64	u64
        // 128-bit	i128	u128
        // arch	isize	usize

        // Rust 的默认值 为i32

        // 整数溢出 :::

        // 浮点类型
        // Rust 的浮点类型是f32和f64，它们的大小分别为 32 位和 64 位。默认类型是f64 因为在现代 CPU 上，它的速度与 大致相同，f32但精度更高。所有浮点类型都是有符号的。
        {
            let x = 2.0; // f64
            let y: f32 = 3.0; // f32
                              // 浮点数根据 IEEE-754 标准表示。该 f32类型是单精度浮点数，并且f64具有双精度。
        }

        // 布尔类型
        {
            let t = true;
            let f: bool = false; // with explicit type annotation
        }

        // 字符类型
        {
            let c = 'z';
            // 我们char使用单引号指定文字，而不是使用双引号的字符串文字 , Rust 的char类型大小为四个字节
            let z: char = 'ℤ'; // with explicit type annotation
            let heart_eyed_cat = '😻';
        }

        // 化合物类型
        {
            // 复合类型可以将多个值组合成一种类型。Rust 有两种原始复合类型：元组和数组。

            // 元组类型
            // 元组是将多个具有多种类型的值组合成一个复合类型的通用方法。元组具有固定长度：一旦声明，它们的大小就不能增加或缩小。
            {
                let tup = (500, 6.4, 1);
                let tup: (i32, f64, u8) = (500, 6.4, 1);

                // 解构取值
                let (x, y, z) = tup;
                // 或者使用 "." 访问
                let five_hundred = tup.0;
                let six_point_four = tup.1;
                let one = tup.2;
            }

            // 数组类型
            {
                // 数组的每个元素都必须具有相同的类型。与某些其他语言中的数组不同，Rust 中的数组具有固定长度 (不可扩大或缩小大小)
                let months = [
                    "January",
                    "February",
                    "March",
                    "April",
                    "May",
                    "June",
                    "July",
                    "August",
                    "September",
                    "October",
                    "November",
                    "December",
                ];
                // i32是每个元素的类型。分号后的数字5 表示数组包含五个元素
                let a: [i32; 5] = [1, 2, 3, 4, 5];

                let a = [3; 5]; // 等价于   let a = [3, 3, 3, 3, 3];
                                // 访问也是通过下标进行访问
                let first = a[0];
                let second = a[1];
            }
        }
    }

    // 函数的使用
    {
        another_function();
        fn another_function() {
            println!("Another function.");
        }

        // 使用参数
        another_function(5);
        fn another_function(x: i32) {
            println!("The value of x is: {x}");
        }

        /**
         * 区别 语句与表达式计算结果
         * 语句是执行某些操作但不返回值的指令。
         * 表达式计算结果值。
         */
        let y = {
            let x = 3; // 语句
            x + 1 // 可以看到这边没用封号 那么他将会返回这个结果到y    如果加上封号就变成一个语句那么就没用返回值了
        };
        println!("The value of y is: {y}");

        // 有返回值的函数
        {
            // 在箭头 ( ->) 之后声明它们的类型
            fn five() -> i32 {
                5 // 隐式返回最后一个表达式   或者可以使用return 显示返回
            }
            let x = five();
            println!("The value of x is: {x}");

            // 参与计算必须要添加入参和类型声明
            fn plus_one(x: i32) -> i32 {
                x + 1;
            }
            let x = plus_one(5);
            println!("The value of x is: {x}");
        }
    }
}
