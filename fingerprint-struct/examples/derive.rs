use blake2::Blake2b512;
use fingerprint_struct::{fingerprint, Fingerprint};
use hex::ToHex;

#[derive(Fingerprint)]
struct Color(u8, u8, u8);

#[derive(Fingerprint)]
struct Point<T>(T, T);

#[derive(Fingerprint)]
enum Shape {
    Background(Color),
    Circle {
        center: Point<f64>,
        radius: f64,
        color: Color,
    },
    Polygon {
        points: Vec<Point<f64>>,
        color: Color,
    },
    Empty,
}

fn main() {
    let payload = vec![
        Shape::Background(Color(0xff, 0xff, 0xff)),
        Shape::Circle {
            center: Point(20.0, 10.0),
            radius: 12.5,
            color: Color(0xff, 0x00, 0xff),
        },
        Shape::Polygon {
            points: vec![
                Point(10.0, 0.0),
                Point(0.0, 10.0),
                Point(-10.0, 0.0),
                Point(0.0, -10.0),
            ],
            color: Color(0x55, 0xff, 0x00),
        },
        Shape::Empty,
    ];

    let hash = fingerprint::<Blake2b512>(payload);
    let hash: String = hash.encode_hex_upper();
    println!("{}", hash);
}
