use core::prelude;

#[allow(dead_code)] // xóa các warning
fn main() {
    print_u8(10u8);
    print_u8(10u64 as u8); //cast
    print_u64(10u64);
    print_generic(10u8);
    print_generic(20u64);
    print_generic("Hello".to_string());
    print_generic("Hellu");
    print_generic(vec![1, 2, 3]);

    // monomorphization -> đại diện cho kiểu dữ liệu chưa biết trước

    let x: Option<i32> = Some(32);
    let y: Option<u32> = None;
    //Option<T>

    //print_generic <&str>("hello"); // đáng lẽ phải có turbofish mà ở đây lại ko cóa
    //print_generic::<u64>(input: 10u8); // lỗi
    print_generic::<&str>("Hellu"); // giống dòng print ở trên

    let point = Point::new(3, 4);
    let point1 = Point::<i32>::new(3, 5);
    //let point1 = Point::new::<i32> (3, 5); // bị lỗi
    let ponit2: Point<u64> = Point { x: 1, y: 2 }; // nó tự hiểu là i32

    return_reference(); // sẽ kéo dài đến hết ctrinh giống clone()

    //let honda = Car {category: "MVP".to_string()};
    //let nissan = Car {category: "MVP".to_string()};

    let vios = Sedan {};
    let speed_vios = vios.speed();
    let bwm = Coupe {};
    let speed_bwm = bwm.speed();

    let circle = Circle { radius: 10.0 };
    let rec = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rec.area());

    let a = get_area(circle); // method là cái trait đã đc khai báo trên
    println!("circle area with trait bound: {}", a);

    // cách dùng đối với square<T>
    let Square: Square<f64> = Square {x: 10.0};
    println!("Square area: {}", Square.area());

    let c = get_area(Square);
    
}

// generic type: kiểu dữ liệu chung, tổng hợp
// type: kiểu dữ liệu
// primitive
// collections
// trait
// generic: chung tổng hợp

fn print_u8(input: u8) {
    println!("Input: {}", input)
}

fn print_u64(input: u64) {
    println!("Input: {}", input)
}
//turbofish: <>
fn print_generic<T: std::fmt::Debug>(input: T) {
    println!("Input: {:?}", input)
}
// Option -> genetic type: T -> enum -> none và some

struct Point<T> {
    // các T này cùng kiểu dữ liệu với nhau
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

struct Pointtwo<T, K> {
    x: T,
    y: K,
}

// đánh dấu lifetime -> kéo dài đến cuối chương trình
fn return_reference<'a>() -> &'a str {
    //<'a> turbofish
    let my_str = String::from("I am a fucking String");
    //&my_str //
    "hello"
}

/*pub struct Car {
    category: String,

}
impl Car {
    fn get_category(&self) {
        println!("Category: {}", self.category);
    }
} */
//thay vì dùng struct như trên
// trait -> định nghĩa 1 object
trait Car {
    // định nghĩa đặc tính của car
    fn get_category(&self) -> String;
    fn speed(&self) -> u32;
    // general
    // chia sẽ các đặc tính -> shared behaviour -> chia cho những kiểu dữ liệu khác nhau
}
//định nghĩa hàng loạt các đặc tính cho 1 object mình muốn
struct Sedan {}
impl Car for Sedan {
    // ghi đè lại các đặc tính chung của Car
    fn get_category(&self) -> String {
        "Bốn bánh".to_string()
    }
    fn speed(&self) -> u32 {
        100
    }
}
struct Coupe {}
impl Car for Coupe {
    fn get_category(&self) -> String {
        "Xe mui trần".to_string()
    }
    fn speed(&self) -> u32 {
        200
    }
}
// nhận xét
// sử dụng các đặc tính chung để mô tả cho object cụ thể
// tính riêng biệt của mỗi object dựa trên đặc tính chung
struct MPV {} // tương tự trên

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}
trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle")
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a Rectangle")
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// nhưng có ràng buộc bởi trait Drawable
// gọi là trait bound: generic type đang ràng buộc một số trait
fn get_area<T: Drawable>(method: T) -> f64 {
    // hàm này với mong muốn sử dụng đặc tính của trait
    method.area()
}

fn get_area_1<T>(method: T) -> f64 where T: Drawable,
{
    // generic function sử dụng trait
    method.area()
} // hàm này tương tự hàm trên
  // cách dùng

fn get_area_2(method: impl Drawable) -> f64 {
    method.area()
} // tương tự hàm trên nhưng ko dùng generic type

// Generic struct
struct Square<T> {
    x: T, // T: primitive string heap
}
impl<T: std::ops::Mul<Output = f64> + Copy> Drawable for Square<T> {
    //T: std::ops::Mul<Output = f64>> -> cho phép generic type nhân với nhau -> 1 cách quá tải toán tử (overload)
    // + Copy -> thì nó sẽ clone cái biến x, và ko gây ra lỗi ownership, sẽ cho cộng nhiều trait
    fn draw(&self) {
        println!("Draw Square")
    }
    fn area(&self) -> f64 {
        self.x * self.x
    }
}

//fn create_shape() -> object mà trait Drawable có implement {  
//    Circle {radius: 5.0}
//}
struct Triangle {
    shape: f64
}

fn create_shape()-> impl Drawable {// ở đây đc vì drawable có impl cho circle
    //Circle {radius: 5.0}
    Square {x: 10f64} // này thì được
    // Triangle{shape: 5.0} // ở đây thì bị lỗi vì ko có impl của drawable
}

fn create_shape_other <T: Drawable> () -> T {
    Circle {radius: 5.0}
}

//SUMMARY
// Generic type
// life time
// trait
// định nghĩa trait
// impl trait
// cách sử dụng.
// tại sao sử dụng trait
// trait bound (function with generic type + trait, struct generic + trait)
// return trait

// chú ý:
// kiểu dữ liệu phải được implement
