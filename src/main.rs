use std::string;

fn main() {
    // sử dụng closure (hàm bí danh) hàm ko tên
    let x = |y| {//ở đây nới chưa parameters {
        println!("hello");
        println!("y = {}", y);
    }; 
    x(19);

    let k = |y : &str| -> String {//ở đây nới chưa parameters {
        y.to_string()
    }; 
    let x = k("VBI");
    println!("x = {}", x);

    //for , iter, iter_mut
    let vec = vec![1, 2, 3,4,5];
    //for i in vec {
    //    println!("value = {}", i);
    //}
    vec.iter().enumerate().for_each(|x| {
        println!("index: {}, value: {}", x.0, x.1)
    });
    let vec1 = vec![1, 2, 3, 4, 5];
    let res: Vec<i32> = vec1.iter().map(|x|{
        x * 2
    }).collect();// khi mà dùng iter -> dùng từng phân tử riêng lẻ nên phải collect lại với nhau
    println!("res: {:?}", res); // :? dùng để format in vector ra màn hình
    
    let mut vec2 = vec![];
    for value in vec1 {
        vec2.push(value * 2)
    }
    println!("cách 2 res: {:?}", vec2); 
    
    let vec3 = [1, 2, 3, 4, 5];
    let res = vec3.iter().max();

    //error
    let vec4 : Vec<i32> = vec![];
    let res = vec4.iter().max();
    println!("res: {:?}", res);

    assert_eq!(count_char_occurrences("Rust is fun", 'u'), 2);
    assert_eq!(count_char_occurrences("Mississippi", 's'), 4);

    //mutable
    let s1 =String::from("hello world"); //string có thể thay đổi đc
    // s1 đang sở hữu dữ liệu "hello world"
    //immutable
    //let s2 = &s1[..]; //reference string -> chỉ đọc được

    //gabage collector 
    let s2 = s1;
    // gán giá trị s1 cho s2 và đồng thời s2 sẽ sở hữu dữ liệu "hello world"
    // theo nguyên tắc ownership của rust: 1 biến chỉ sở hữu 1 giá trị tương ứng
    // "helloworld" đổi chủ mới -> xóa s1: drop s1
    // s1 move value to s2
    println!("S2: {}", s2);

    // rust primitive: u32, u64
    // collections: vector, string

    // pointer: trỏ địa chỉ của biến đóa

    //let s3 = s1; 
    // vi phạm nguyên tắc của ownership -> s1 đã bị xóa rùi
    //

    //ownership -> biến ko sử dụng -> xóa lập tức mà ko cần chuowgn trình làm -> đỡ tốn bộ nhớ
    // {} : scope
    let x = 10; // global
    {
        let x = "HELLO";
        println!("Y: {}", x); // trong scope -> local
    }
    println!("y: {}", x);
    // rust: out of scope -> value drop

    // pointer: trỏ địa chỉ của biến đóa
    let x = 42;
    let y = 43;
    let var1 = &x;
    println!("var1: {}", &var1); // pointer này mang địa chỉ nhưng khi print là giá trị = 42
    // var hoặc *var hoặc &var đều đc
    println!("var1: {:p}", var1); // in ra địa chỉ
    function_a();
     
    let s3 = String::from("hello");//
    println!("s3 address: {:?}", s3.as_ptr());// bộ nhớ động
    // stack và heap
    // stack: địa chỉ ổ nhớ để lưu giá trị cố định
    // heap: ổ nhớ động

    // Primitive: default là stack -> biết size
    //collections (string, vec) có bỏ vào stack được hay khum ?
    // không biết size ở compile time
    //lưu ở heap -> nội dung
    // stack: con trỏ ptr, len, capacity
    let s4 = &s3;
    println!("s4 address: {:?}", s4.as_ptr());
    // s4 và s3 cùng địa chỉ
    println!("s3: {}", s3);// tham chiếu -> s4 đang trỏ cùng 1 địa chỉ với s3 -> chưa ảnh hưởng tới giá trị

    let s3 = String::from("hello");
    println!("s3 address: {:?}", s3.as_ptr());
    let s4 = s3;
    println!("s4 address: {:?}", s4.as_ptr()); // ở đây s4 và s3 vẫn cùng địa chỉ

    let mut s6 = String::from("hell");
    println!("Len: {}, Capacity: {}", s6.len(), s6.capacity());
    s6.push_str("huhu");
    println!("Len: {}, Capacity: {}", s6.len(), s6.capacity());// capacity == 8
    s6.push_str("heeeehu");
    println!("Len: {}, Capacity: {}", s6.len(), s6.capacity());// capacity == 16

    let mut s7 = String::from("hey");
    let s8 = s7.clone(); // s8 và s7 riêng lun dù s8 mang tất cả mọi thứ giống s7
    // clone thì gây tốn bộ nhớ
    println!("s7 address: {:?}", s7.as_ptr());
    println!("s8 address: {:?}", s8.as_ptr()); 
    println!("s7: {}", s7);
    //reference = borrowing
    let s9 = &s7;
    //shared memory - immutable nên s9.push_str("world"); ko được
    println!("s7 address: {:?}", s7.as_ptr());
    println!("s9 address: {:?}", s9.as_ptr());

    s7.push_str("sus");// s7 thay đổi, và s7 thì có quyền thay đổi dù cho s9 mượn,
    //println!("s9: {}", s9);

    let s10 = &s7;
    let s11 = &s7;// có thể shared reference nhiều lần. 

    //mutable reference
    // có thể thay đổi giá trị 
    //nhưng owner phải share quyền thay đổi mut
    let mut s12 = &mut s7;
    s12.push_str("hahahah");
    println!("s12: {}", s12);// s7 cho phép s12 thay đổi giá trị -> s7 thay đổi lun
    // && = &
    // chỉ 1 mut ref cho 1 gtri nhất định trong 1 thời điểm
    // nhìu mut ref cho phép sử dụng (đọc)
    // ko thể có 1 mut ref khi mà có 1 mut ref tham chiếu tới cùng 1 giá trị
    let s = String::from("hu");
    //let s2 = &s;
    change1(&mut s);
    //println!("s2: {}", s2); //mất ownership
    change2(s);
    //println!("s: {}", s); // mất ownership
    
}

fn change1 (some_str: &mut String){
    some_str.push_str("uida");
}

fn change2 (mut some_str: String){
    some_str.push_str("uida");
}

fn function_a() {
    println!("hello");
    let x = 20;

}

fn count_char_occurrences(string : &str, ch: char) -> usize {
    string.chars().into_iter().filter(|x| x == &ch).count()// filter -> bool
}