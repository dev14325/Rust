// enum Direction {
//     North,East,South,West,
// }

// fn main(){
//     move_around(Direction::North);

// }

// fn move_around(direction : Direction){
//     // implement logic to move a character around
// }


// enum with values
// Define an enum called Shape
enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    // calculates the area of the shape 
    match shape {
        Shape::Circle(r) =>3.14*r*r,
        Shape::Square(a) =>a*a,
        Shape::Rectangle(w,h) =>w*h,
    }
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

println!("Area of circle : {}",calculate_area(circle));
println!("Area of square : {}",calculate_area(square));
println!("Area of rectangle : {}",calculate_area(rectangle));
    
}