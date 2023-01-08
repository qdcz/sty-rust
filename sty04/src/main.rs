fn main() {
    // 条件判断
    {
        // if表达式
        let number = 3;
        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }

        // if的条件必须是布尔值  以下为错误用法
        // let number = 3;
        // if number {
        //     println!("number was three");
        // }

        // else if  跟其他语言使用一样

        // 使用if将表达式赋值给变量
        {
            let condition = true;
            let number = if condition { 5 } else { 6 };
            println!("The value of number is: {number}");
        }
        /**
         * 注意:
         * 代码块的计算结果是其中的最后一个表达式，数字本身也是表达式。
         * 在这种情况下，整个if表达式的值取决于执行的代码块。这意味着可能来自每个分支的结果的值if必须是同一类型
         *
         * 以下是报错写法
         */
        {
            // let condition = true;
            // let number = if condition { 5 } else { "six" };
            // println!("The value of number is: {number}");
        }

        // 循环写法
        {
            // 重复代码loop
            let mut counter = 0;
            let result = loop {
                counter += 1;
                if counter == 10 {
                    break counter * 2; // 使用 break带有值的关键字counter * 2,循环后，我们用分号结束给 赋值的语句result
                }
            };
            println!("The result is {result}"); // 20

            // 跳出到外层循环位置 而不是直接结束
            {
                let mut count = 0;
                'counting_up: loop {
                    println!("count = {count}");
                    let mut remaining = 10;
                    loop {
                        println!("remaining = {remaining}");
                        if remaining == 9 {
                            break;
                        }
                        if count == 2 {
                            break 'counting_up;
                        }
                        remaining -= 1;
                    }
                    count += 1;
                }
                println!("End count = {count}");
            }

            // while循环
            {
                let mut number = 3;
                while number != 0 {
                    println!("{number}!");
                    number -= 1;
                }
                println!("LIFTOFF!!!");
            }

            // 使用for遍历
            {
                let a = [10, 20, 30, 40, 50];
                for element in a {
                    println!("the value is: {element}");
                }

                for number in (1..4).rev() {
                    println!("{number}!");
                }
                println!("LIFTOFF!!!");
            }
        }
    }
}
