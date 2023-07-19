use std::{collections::HashMap, iter::Successors};
fn main() {
    let x = (10, "siu", 6.5); // tuple

    // struct -> đối tượng -> nhìu hình thái biểu diễn cho đối tượng.
    // đây là kiểu dữ liệu đc định nghĩa bởi người lập trình
    // hình thái cụ thể
    // instance: 1 thực thể
    // có thể để trong main hoặc ngoài main
    struct class {
        name: String,
        subject: String,
        teacher: String,
    }
    let student_a: Students = Students { 
        class: 1, 
        symbol: "Hi".to_string(), 
        grade: 10.0,
    }; 
    let student_b: Students = Students { 
        class: 1, 
        symbol: "Fuck yu".to_string(), 
        grade: 10.0,
    }; 
    // Struct sẽ được lưu vào stack hay heap thì sẽ phụ thuộc vào kiểu dữ liệu trong struct
    let class_1: u8 = 1;
    let symbol = "bye".to_string();
    let grade = 8.0;
    let student_c = Students {class: class_1, symbol, grade};
    

    let class_of_c = student_c.get_class();
    println!("class of Duong: {}", class_of_c);

    let mut student_d = Students::new_student();// dòng này giống dòng 29 vì đề khởi tạo thực thể
    let student_e = Students::new_student();
    // :: và .

    let fail = student_d.clone().get_symbol(); // hàm này lấy ownership của student_d
    let success = student_e.get_symbol2(); 

    println!("student_d: {:?}", student_d);// dùng clone như fail hoặc reference như success vì biến d đã bị xóa khi lấy ownership

    //dùng shared reference
    println!("student_e: {:?}", student_e);// print với :? sẽ in toàn bộ mọi thứu

    // mutable reference
    student_d.set_grade(20.0);
    println!("student d grade: {}", student_d.grade);

    let stu = vec![student_a, student_b];

    // Enum
    let direction = Direction::West;
    let res = direction.convert_string();
    println!("direction: {:?}", res);
    // struct thì dùng nhìu hơn còn enum thì dùng cho 

    let shape = shape::Circle { rad: (2.0) };
    let su = shape::Round (2.0);
    let rect = shape::Rectangle { width: (10.0), length: (20.0) };
}

#[derive(Debug, Clone)]// clone ở trên thì phải khai clone dưới đây
struct Students{
    class: u8,
    symbol: String,
    grade: f64,
}
// mô tả hành vi đối với 1 đối tượng chung
impl Students {
    //constructor
    // khởi tạo instance
    pub fn new_student() -> Students { // pub: public, ko có thì là private
        Students { class: (10), symbol: ("sleep".to_string()), grade: (6.0) }
    }
    //method: get, set, print ....
    fn get_class(self) -> u8 {
        self.class // self là chính nó, là thực thể
    }
    fn get_symbol(self) -> String {
        "OK".to_string()
    }
    fn get_symbol2(&self) -> String {
        "OK".to_string()
    }
    // + value nào đó
    fn set_grade(&mut self, new_grade: f64) {
        self.grade += new_grade;
    }

    //hàm gọi hàm trong imp
    fn test(self) -> String {
        self.get_symbol()
    }

    fn get_attr(&self, input: &str) -> String {
        match input {
            "class" => self.class.to_string(),// ở đây to_string() giống như string.clone() nên ko bị ownership
            //"symbol" => self.symbol, // bị lỗi ownership
            "grade" => self.grade.to_string(),
            _ => todo!()
        }
    }
}

enum Direction {
    West,
    East,
    South,
    North
}
impl Direction {
    fn convert_string(self) -> String {
        match self {
            Direction::West => "West".to_string(),
            Direction::East => "East".to_string(),
            Direction::South => "South".to_string(),
            Direction::North => "North".to_string()
        }
    }
}

struct DirectionStr {
    direct: String,
}

// Enum dành cho việc định nghĩa 1 tập hợp của 1 object mang tính giới hạn, 
// các lựa chọn là hữu hạn
enum shape {
    Rectangle {width: f32, length: f32},
    Circle {rad: f32},
    Round (f32)
}

enum UNDERSTANDING {
    A_BIT,
    MAKE_SENCE,
    NO
}

struct Node {
    value: f32,
    next: &'static Node
    // hoặc là next: &'a Node<'a>
}

/*
    pub enum Option<T> {
        None,
        Some(T),
    }
*/

// note:
// dangling reference -> trỏ tới biến mượn, mượn biến bị drop
// self, &self, &mut self 
// self -> instance
// cách lấy trường dữ liệu của object (instance.<tên dữ liệu>)
// new -> constructor
// self và Self
// enum

// get_attr
// khi nào dùng enum và struct
// partial move
// enum có thể define struct hay ko và ngược lại