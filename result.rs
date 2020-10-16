fn my_function() {}
trait MyTrait {
    fn my_trait(&self);
}
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn add(&self, oth: Point) -> Point {
        Point {
            x: self.x + oth.x,
            y: self.y + oth.y,
        }
    }
}
#[cfg(test)]
mod should {
    struct FakeMyTrait;
    impl MyTrait for FakeMyTrait {
        fn my_trait(&self) {}
    }
    #[test]
    fn test_my_function() {}
    #[test]
    fn test_Point_new() {}
    #[test]
    fn test_Point_add() {}
}
