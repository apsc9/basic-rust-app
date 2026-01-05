
mod shapes;
mod calculator;
use shapes::*;
use calculator::*;

fn rectangle_example () {
    let width = 10.0;
    let height = 20.0;

    let mut rect = Rectangle::new(width, height).unwrap();

    let new_width = 30.0;
    let new_height = 40.0;

    let res = rect.set_width(new_width);
    assert!(res.is_ok());

    let res = rect.set_height(new_height);
    assert!(res.is_ok());

    let new_height = -14.0;
    let res = rect.set_height(new_height);

    assert_eq!(
        res.err(),
        Some(Error::InvalidHeight)
    );
}

fn circle_example () {
    let radius = 10.0;
    let mut circle = Circle::new(radius).unwrap();

    let new_radius = 15.5;

    let res = circle.set_radius(new_radius);
    assert!(res.is_ok());

    let new_radius = -2.5;
    let res = circle.set_radius(new_radius);

    assert_eq!(
        res.err(),
        Some(Error::InvalidRadius)
    )
}

fn calculator_example () {
    let x = -4;
    let y = 9;

    let mut calc = Calculator::new();

    let addition = calc.add(x, y);
    let subtraction = calc.subtract(x,y);
    let multiplication = calc.multiply(x,y);

    assert_eq!(addition, Some(5));
    assert_eq!(subtraction, Some(-13));
    assert_eq!(multiplication, Some(-36));

    calc.repeat(1);
    calc.repeat(0);
    let history = calc.history();

    let expected = "0: -4 + 9 = 5\n1: -4 - 9 = -13\n2: -4 * 9 = -36\n3: -4 - 9 = -13\n4: -4 + 9 = 5\n";

    assert_eq!(history, expected);

    calc.clear_history();
    assert_eq!(calc.history(), "");

    assert_eq!(calc.repeat(1), None);
    assert_eq!(calc.history(), "");

}

fn main() {
    rectangle_example();
    circle_example();
    calculator_example();
    println!("All examples executed successfully!");
}
