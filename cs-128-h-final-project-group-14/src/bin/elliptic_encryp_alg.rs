use rand::rngs::OsRng;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub infinity: bool,
}

impl Point {
    pub fn infinity() -> Self {
        Point { x: 0, y: 0, infinity: true }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EllipticCurveAlg {
    pub message: Vec<u8>,
    pub private_key: i64,
    pub public_key: Point,
    pub encrypted: Vec<(Point, Point)>,
}

const A: i64 = 2;
const P: i64 = 97;

const GENERATOR: Point = Point {
    x: 3,
    y: 6,
    infinity: false,
};

impl EllipticCurveAlg {

    pub fn new(message_string: String, private_key: i64) -> Self {

        let public_key = Self::scalar_mult(private_key, GENERATOR);

        EllipticCurveAlg {
            message: message_string.into_bytes(),
            private_key,
            public_key,
            encrypted: Vec::new(),
        }
    }


fn mod_inv(x: i64) -> i64 {
        for i in 1..P {
            if (x * i) % P == 1 {
                return i;
            }
        }
    0
}

fn point_add(p1: Point, p2: Point) -> Point {

    if p1.infinity { return p2; }
    if p2.infinity { return p1; }

    if p1.x == p2.x && (p1.y + p2.y) % P == 0 {
        return Point::infinity();
    }

    let lambda = if p1 == p2 {
        let num = (3 * p1.x * p1.x + A) % P;
        let den = Self::mod_inv(2 * p1.y % P);
        (num * den) % P
    } else {
        let num = (p2.y - p1.y) % P;
        let den = Self::mod_inv((p2.x - p1.x) % P);
        (num * den) % P
    };

    let x3 = (lambda * lambda - p1.x - p2.x) % P;
    let y3 = (lambda * (p1.x - x3) - p1.y) % P;

    Point {
        x: (x3 + P) % P,
         y: (y3 + P) % P,
         infinity: false,
    }
}

fn scalar_mult(mut k: i64, mut point: Point) -> Point {
        let mut result = Point::infinity();

        while k > 0 {
            if k % 2 == 1 {
                result = Self::point_add(result, point);
            }
            point = Self::point_add(point, point);
            k /= 2;
        }

    result
}

pub fn encrypt(&mut self) {

        self.encrypted.clear();

        let mut rng = OsRng;

        for &byte in &self.message {

            
            let m_point = Self::scalar_mult(byte as i64, GENERATOR);

            let k: i64 = rng.gen_range(1..P);

            let c1 = Self::scalar_mult(k, GENERATOR);
            let shared = Self::scalar_mult(k, self.public_key);

            let c2 = Self::point_add(m_point, shared);

            self.encrypted.push((c1, c2));
        }
    }
}
