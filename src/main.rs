use std::collections::btree_map::Values;

fn main() {
    println!("Hello, world!");
    let x: i32 = 100i32;
    // i - integer. u - unsigned, f - float
    let y: f64 = 10.5;
    let boo = false;
    let array: [i32; 3] = [10, 20, 30];
    println!("array 1: {}, {}", array[0], y);
    println!("array 4: {}", array[2]);
    let tuple1 = (1, 10);
    let tuple1 = (10, 10, 10.0, "s");
    println!("tuple1: {}", tuple1.0);
    println!("tuple1: {}", tuple1.3);
    let ho_dung: &str = "10";
    let mut a = 10;
    a = a + 10;
    println!("print a: {}", a);
    const PI: f64 = 3.14;
    const NUMBER_CASE: u32 = 1;
    let mut s = String::new();
    println!("string is empty: {}", s.is_empty());
    s.push('h');
    println!("print s: {}", s);
    let mut s = String::from("Hello_World");
    println!("print s: {}", s);
    //cast
    //reference str; -> &str chỉ có quyền đọc
    // std::mem::size_of::<char>();
    let s2 = "Hello World";
    s = s + s2;
    println!("print s: {}", s);

    println!("result is: {}", &s2[0..5]);
    let mut s3 = "YO";
    // s3 = s3 + "Dang" -> ko được nha

    //conversion &str -> string
    let conversion_string = "VBI".to_string();
    let convesion_str = &conversion_string;
    let coversion_str = &&&&conversion_string.as_str();

    let byte = conversion_string.as_bytes();
    println!("result is: {:?}", byte);
    let conversion_str = &*conversion_string; // giống as_str() nhưng ko dùng thường xuyên

    let mut s = String::new();
    println!("{}", s);

    let mut s = "Teoe";
    println!("{}", s);
    let s_for = format!("{}", "Hello nha");

    let x = true;
    if x {
        println!("hi");
    } else {
        println!("bar");
    }

    // Pattern Matching
    // match == switch case nhưng powerfull hơn 1 xí
    match x {
        true => println!("cook"),
        false => println!("cum")
    }

    //default match
    let number = 10;
    match number {
        5 => println!("hello: {}", number),
        10 => println!("hup: {}", number), 
        5 => println!("hello invalid"), //nó sẽ ưu tiên thằng đầu tiên
        _ => todo!()// phần còn lại
    }
    let tuple = (10, 10);
    match tuple {
        (10, 10) | (20, 20) => println!("siuuuu"), // | if 
        _ => todo!()
    }

    let vec = vec![1,23,4,5,6];
    //for value in vec {
    //    println!("value: {}", value);
    //}
    
    for value in vec.iter() {
        println!("Value: {}", value);
    }

    for index in 0..vec.len(){

    }
    // phân biệt giữa for bình thường và iter() hoặc iter_mut(), into_iter();
    let max = vec.iter().max().unwrap(); // tìm max
    println!("Max: {}", max); 
    func_x();
    func_y(String::from("ADU"));
    let j = func_z(String::from("ADU"));
}

fn func_x() {
    println!("ABC");
}
fn func_y(s: String) {
    println!("ABC: {}", s);
}
fn func_z(s: String) -> String {
    println!("ABC: {}", s);
    return String::from("Hup");
}