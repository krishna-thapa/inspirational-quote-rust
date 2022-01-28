/// The doc for doo
pub fn foo() {
    println!("The foo function");
}

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test{
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}