fn my_function() {

}

trait MyTrait {
    fn my_trait(&self);
}

struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    
    fn add(&self, oth: Point) -> Point {
        Point {
            x: self.x + oth.x,
            y: self.y + oth.y
        }
    }
}