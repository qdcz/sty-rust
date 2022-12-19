/**
 * 猜谜游戏程序的第一部分将要求用户输入、处理该输入并检查输入是否符合预期形式。首先，我们将允许玩家输入一个猜测。
 */

//  io输入/输出库,该io库来自标准库，称为std
use std::io;

use rand::prelude::*;
use rand::Rng;



// main函数是程序的入口点：
fn main() {

    // 随机数
    // let x: u8 = random();
    // println!("{}", x);

    // println!是一个将字符串打印到屏幕的宏：
    println!("Guess the number!");

    println!("Please input your guess.");

    // 创建一个变量来存储用户输入，如下所示：
    // ::new表明它new是该String类型的关联函数
    let mut guess = String::new();

    // 接收用户输入  理解为  io.stdin
    // read_line标准输入句柄上的方法以获取用户的输入
    // &表示此参数是一个引用，它为您提供了一种方法，让您的代码的多个部分访问一个数据，而无需将该数据多次复制到内存中。
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    // 一个在当前执行线程本地并由操作系统播种的随机数生成器
    // start..=end是包含下限和上限
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
}
