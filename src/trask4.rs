
#[test]// Fix the error with the use of define_x
fn main(){
    let x = define_x();
    println!("{}, world", x );
}
pub fn define_x() -> &'static str {
    "hello"
}