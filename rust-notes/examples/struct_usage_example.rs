fn main() {
    example_1();
    example_tuples();
    example_structure();
    example_println();
    example_dbg();
}

fn example_1() {
    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    let width1 = 30;
    let height1 = 50;

    println!(
        "The erea of rectangle is {} square pixels.",
        area(width1, height1)
    )
}

fn example_tuples() {
    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    let rect1 = (30, 50);

    println!("The erea of rectangle is {} square pixels.", area(rect1))
}

fn example_structure() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The erea of rectangle is {} square pixels.", area(&rect1))
}

fn example_println() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    area(&rect1);

    println!("rect1 is {rect1:?}")
}

fn example_dbg() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("{scale}");

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    area(&rect1);

    dbg!(&rect1);
}
