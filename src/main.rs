use std::{fmt::Display, num::NonZeroI128};

fn main() {
    let circle = Circle { radius: 10.0 };
    println!("circle: {:?}", circle);
    println!("Circle draw: {}", circle.area());

    let mut counter = Counter { x: 0.0 };
    println!("Counter next: {:?}", counter.next());
    println!("Counter next: {:?}", counter.next());

    let mut counterA = CounterAssociated { x: 0 };
    println!("Counter nextA: {:?}", counterA.next());
    println!("Counter nextA: {:?}", counterA.next());

    let mut vec: Vec<u32> = vec![1, 2, 3, 4];
    let res = vec.next();
    println!("Next: {:?}", res);

    let mut vec1: Vec<u32> = vec![1, 2, 3, 4];
    let res: u32 = vec1.next().unwrap();
    println!("Next: {:?}", res);

    println!("Create shape: {:?}", create_shape());
    println!("Create shape 2: {:?}", create_shape_2());

    let circle1 = Circle { radius: 10.0 };
    let rec = Rectangle {
        width: 5.0,
        height: 10.0,
    };
    //let shape = vec![ circle1, rec]; // ko đc
    // làm như thế nào để lưu đc 2 object circle1 và rec
    // -> cho nó vào box
    let tri = Triangle {};
    let shape: Vec<Box<dyn Shape>> = vec![Box::new(circle1), Box::new(rec)];
    //let shape: Vec<Box<dyn Shape>> = vec![ Box::new(circle1), Box::new(rec), Box::new(tri)]; // này thì ko đc vì shape chưa impl cho Triagle
    // ta xem trait shape như 1 kiểu dữ liệu
    // biến trait thành 1 kiểu dữ liệu
    // trait object
    //static dispatch
    // static -> tĩnh / dispatch -> execute 1 hàm

    for i in shape {
        let res = i.area();
        println!()
    }

    // cách sử dụng static dispatch
    draw_static(&circle);
    draw_dynamic(&circle);

    // chuyển kiểu dữ liệu
    let res = String::from(ErrorCus::NotFound);
    println!("res: {}", res);
    let res2: String = ErrorCus::NotFound.into();
    println!("res2: {}", res2);
    let res3 = String::from_f_1(Box::new(ErrorCus::NotFound));

    let student = Student {
        grade: 10,
        name: "Dang".to_string(),
    };
    println!("Student: {}", student);
}

//static dispatch
// áp dụng trait bound
// nó biết rằng có circle và rec
// drawable thì có impl cho circle, recư

// ở compile time, thì biết đc object đã xác định
// như là struct: Circle, rectangle

fn draw_static<T: Drawable>(shape: &T) {
    //trait bound
    shape.draw()
}
// -> đây là bản chất của static dispatch
fn draw_static_real(shape: &impl Drawable) {
    shape.draw()
}

fn draw_static_circle(shape: &Circle) {
    shape.draw()
}

fn draw_static_rectangle(shape: &Rectangle) {
    shape.draw()
}

//dynamic dispatch
// dùng trưc tiếp cái object là trait lun
fn draw_dynamic(shape: &dyn Drawable) {
    // nếu như không cso dyn thì hàm ko biết trait drawalbe có impl cho kiểu dữ liệu nào hay ko
    // nó chỉ biết khi chạy runtime -> mới biết drawable impl cho th nào
    // thêm dyn để Drawable biến thành trait object
    shape.draw()
}

//static -> trait bound
// dynamic -> dyn

struct Triangle {}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

// supertraits -> những thằng nào mún dùng trait Shape thì phải
// impl trait Drawable sẵn
// nó khá giống kế thừa
trait Shape: Drawable + std::fmt::Debug {
    fn area(&self) -> f64;
}

trait Drawable {
    fn draw(&self);
    //fn area(&self) -> f64;
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle")
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a Rectangle")
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn create_shape() -> impl Shape {
    // nó biết cái kích thước của object
    Circle { radius: 3.0 } // trả về kiểu dữ liệu mà trait impl
}
// Box: smart pointer -> lưu thông tin lưu vào heap
// kiểu như String::new(), Vec::new()
fn create_shape_2() -> Box<dyn Shape> {
    // dyn (-> dynamic) thì ko biết kích thước
    // nên phải dùng Box
    // graph cái object shape với cái smart pointer
    Box::new(Circle { radius: 4.0 })
}
// 2 cái create shape thì kết quả giống nhau
fn foo() -> &'static str {
    "Hello"
}
//fn create_shape_3 () -> &'static dyn Shape {
//    &Box::new(Circle {radius: 4.0})
//}

// Associated type -> gần giống generic type
// nhưng nó có nhiều lợi thế hơn khỉ chỉ sử dụng gen type
/*
pub trait Iterator {
    type Item;
}
*/
pub trait Iterator<T> {
    fn next(&mut self) -> T;
}
pub struct Counter<T> {
    x: T,
}

impl<T: std::ops::Add<f64, Output = T> + Copy> Iterator<T> for Counter<T> {
    fn next(&mut self) -> T {
        self.x = self.x + 1.0;
        self.x
    }
}

pub struct CounterAssociated {
    x: u32,
}

pub trait IteratorA {
    type Item;
    // thay vì dùng generic type T
    // thì mình dùng Associated Type
    fn next(&mut self) -> Self::Item;
}

impl IteratorA for CounterAssociated {
    type Item = u32;
    fn next(&mut self) -> Self::Item {
        self.x = self.x + 1;
        self.x
    }
}

trait IteratorVec {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl IteratorVec for Vec<u32> {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        Some(10)
    }
}

struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}

// C chứa A, B
// trait bound
fn difference<A, B, C>(container: &C) -> i32
where
    // có thể ghi là fn difference <A, B, C: Contains <A, B>> (container: &C) -> i32
    C: Contains<A, B>,
{
    container.last() - container.first()
}

// Bây giờ thay generic type thành associative type chỉ cần 1 gen type: C
struct Container_Asso(i32, i32);

//interface -> shared behaviour cho các kiểu dữ liệu khác
trait Contains_Asso {
    type A;
    type B;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains_Asso for Container_Asso {
    type A = i32;
    type B = i32;
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}

fn difference_Asso<C: Contains_Asso>(container: &C) -> i32 {
    container.last() - container.first()
}

// static dispatch
// dynamic dispatch

#[derive(Debug)]
// super trait
enum ErrorCus {
    NotFound,
    FailToCreate,
}

// thiếu 2 impl là Display và Debug
impl std::error::Error for ErrorCus {}

impl std::fmt::Display for ErrorCus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
// làm sao mình có thể chuyển đc kiểu dạng từng phần tử của enum sang string
// Enum -> string
impl From<ErrorCus> for String {
    fn from(value: ErrorCus) -> Self {
        match value {
            ErrorCus::NotFound => "not found".to_string(),
            ErrorCus::FailToCreate => "fail to create".to_string(),
        }
    }
}

trait FromFake<T> {
    fn from_f(value: T) -> Self;
}
impl FromFake<ErrorCus> for String {
    fn from_f(value: ErrorCus) -> Self {
        match value {
            ErrorCus::NotFound => "not found".to_string(),
            ErrorCus::FailToCreate => "fail to create".to_string(),
        }
    }
}

trait FromFake_1<T> {
    fn from_f_1(value: T) -> Self;
}

impl FromFake_1<Box<dyn std::error::Error>> for String {
    fn from_f_1(value: Box<dyn std::error::Error>) -> Self {
        value.to_string()
    }
}

struct Student {
    grade: u8,
    name: String,
}
impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!()
        //write!(f, "{},{}", self.grade, self.name)
        format!("{}, {}", self.grade, self.name).fmt(f) // 1 trong 2 cách như vậy
    }
}

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}

// overview
// static dispatch -> sai ở complile time
// -> trait bound
// dynamic dispatch -> sai ở run time
// -> dyn

// cách implement 1 trait nào đó (từ thư viện) hoặc từ customized
// C1: mình làm sẵn hết, mình đưa ra unit test -> mn viết logic
// C2: chỉ đưa ra yêu cầu và gợi ý -> tự mn implement
