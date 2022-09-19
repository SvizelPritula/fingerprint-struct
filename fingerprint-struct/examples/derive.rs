use digest::Update;
use fingerprint_struct::Fingerprint;

#[derive(Fingerprint)]
struct Color(u8, u8, u8);

#[derive(Fingerprint)]
struct Ball {
    pub x: isize,
    pub y: isize,
    pub fill_type: FillType,
}

#[derive(Fingerprint)]
#[allow(dead_code)]
#[repr(u16)]
enum FillType {
    Color(Color),
    Gradient(Color, Color),
    Shape { name: String, offset: u32 },
    Transparent,
}

fn main() {
    let point = Ball {
        x: 100,
        y: -50,
        fill_type: FillType::Gradient(Color(123, 36, 28), Color(91, 44, 111)),
    };

    let mut hasher = MockDigest::default();
    point.fingerprint(&mut hasher);

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
    #[inline]
    fn update(&mut self, data: &[u8]) {
        self.bytes.extend_from_slice(data)
    }
}
