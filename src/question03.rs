pub trait HasArea {
    fn area(&self) -> f64;
}

struct Rectangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { 
        println!("length: {}", self.length);
        println!("height: {}", self.height);
        return self.length * self.height;
    }
}


fn area<T: HasArea>(t: &T) -> f64 { t.area() }


fn main() {
    let rectangle = Rectangle { length: 7.0, height: 8.0 };
    println!("Area: {}", area(&rectangle));

}