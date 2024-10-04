#[test]
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); // Змінна `x` в межах блоку дорівнює 12
    }

    assert_eq!(x, 5); // Поза блоком, початкова змінна `x` дорівнює 5

    let x = 42;
    println!("{}", x); // Prints "42".
}
