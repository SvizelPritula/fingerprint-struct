use digest::Update;
use fingerprint_struct::Fingerprint;

#[derive(Fingerprint)]
struct Color(u8, u8, u8);

#[derive(Fingerprint)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Fingerprint)]
enum Shape {
    Background,
    Line(Point, Point),
    Circle { center: Point, radius: u32 },
}

#[derive(Fingerprint)]
struct Object {
    shape: Shape,
    color: Color,
}

fn main() {
    let shapes = vec![
        Object {
            shape: Shape::Background,
            color: Color(0xff, 0xff, 0xff),
        },
        Object {
            shape: Shape::Circle {
                center: Point { x: 0., y: 0. },
                radius: 20,
            },
            color: Color(0xff, 0x00, 0x33),
        },
        Object {
            shape: Shape::Line(Point { x: 0., y: 20.5 }, Point { x: 0., y: -20.5 }),
            color: Color(0x22, 0x22, 0x22),
        },
    ];

    let mut hasher = MockDigest::default();
    shapes.fingerprint(&mut hasher);
    println!("{:?}", hasher.as_bytes());
}

#[derive(Default)]
struct MockDigest {
    bytes: Vec<u8>,
}

impl MockDigest {
    fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

impl Update for MockDigest {
    fn update(&mut self, data: &[u8]) {
        self.bytes.extend_from_slice(data)
    }
}
