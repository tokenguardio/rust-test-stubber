fn my_function() {}
trait MyTrait {
    fn my_trait(&self);
}
#[cfg(test)]
mod should {
    struct FakeMyTrait;
    impl MyTrait for FakeMyTrait {
        fn my_trait(&self) {}
    }
    #[test]
    fn test_my_function() {}
}
