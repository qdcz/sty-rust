fn main() {
    println!("切片类型，他是一种引用，并无所有权");

    let v = vec![1, 2, 3, 4, 5];

    v.iter().for_each(|x| println!("{x}"));
    // or
    for x in &v {
        println!("{x}");
    }
    v.iter().for_each(|x| println!("{x}"));

    let s = "hello";
    {
        let s = "hello";
    }
}

/**
 * uszie：https://doc.rust-lang.org/nightly/std/primitive.usize.html
 * 指针大小的无符号整数类型 此原语的大小是引用内存中任何位置所需的字节数。例如，在 32 位目标上，这是 4 个字节，在 64 位目标上，这是 8 个字节。
 */
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
