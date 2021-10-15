use generics::Point;

fn main() {
    let p = Point { x: 5.0, y: 10.0 };

    println!("{:?}", p.area());
}
