fn main() {
    let s1 = String::from("不可改引用");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);


    let mut s2 = String::from("可变引用");
    change(&mut s2);

    
    test();
}


// 该&s1语法让我们创建一个引用但s1 不拥有它的值。因为它不拥有它，所以当引用停止使用时，它指向的值不会被删除。
fn calculate_length(s: &String) -> usize {
    s.len()

    // 且他是不可借用（被修改的，否则会报错）
    // s.push_str(", world"); // error
}

// 可修改引用的值（但是只能对一个值的可变引用 不能同时存在两个 如下：）
fn change(s: &mut String) {
    s.push_str(", world");
    println!("可修改引用的值 is {}.", s);

    // 不能同时存在引用两个可变值 如果要这么做 那么要新建一个范围作用域 如 30-33 line
    // let r1 = &mut s;
    // let r2 = &mut s;

    {
        // let r1 = &mut s;  // 当我们有一个指向相同值的不可变引用时，我们也不能有一个可变引用。
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    // let r2 = &mut s;
}

fn test(){
    let mut s = String::from("可变和不可同时引用使用");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);


    // 引用的范围从引入它的地方开始，一直持续到最后一次使用该引用。例如，此代码将编译，因为不可变引用的最后一次使用println!发生在引入可变引用之前
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // r1 r2 打印释放掉引用 才可进行下面mut引用 
    let r3 = &mut s; // no problem
    println!("{}", r3);
}


// 悬挂引用
fn hangReferences() -> &String{
    let s = String::from("hello");

    // 会报错  因为s是在hangReferences里面创建的，当 的代码执行hangReferences完， s就会被释放。
    // 但是我们试图返回对它的引用。这意味着该引用将指向一个无效的String. 那可不行！Rust 不会让我们这样做。
    // 我们可以这么做  参考 66-70 line
    &s
}

fn no_dangle() -> String {
    let s = String::from("hello");

    // 所有权被移出，没有任何东西被释放
    s
}
