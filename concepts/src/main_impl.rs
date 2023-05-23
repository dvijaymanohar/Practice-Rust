trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f32,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!(
            "Drawing a Circle with area: {}\n",
            3.14 * self.radius * self.radius
        );
    }
}

struct Rectangle {
    length: f32,
    breadth: f32,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!(
            "Drawing a Rectangle with area: {}\n",
            self.length * self.breadth
        );
    }
}

fn draw_object(obj: &dyn Drawable) {
    obj.draw();
}

fn main() {
    let rect = Rectangle {
        length: 10.0,
        breadth: 20.0,
    };

    draw_object(&rect);

    let circle = Circle { radius: 10.0 };

    draw_object(&circle);

    return;
}
