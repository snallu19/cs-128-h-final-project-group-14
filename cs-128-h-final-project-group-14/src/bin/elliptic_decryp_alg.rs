

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
pub struct EllipticDecryptAlg {
    pub private_key: i64,
    pub encrypted: Vec<(Point, Point)>,
    pub decrypted: String,
}

const A: i64 = 2;
const P: i64 = 97;

const GENERATOR: Point = Point {
    x: 3,
    y: 6,
    infinity: false,
};

impl EllipticDecryptAlg {

    pub fn new(data: Vec<(Point, Point)>, private_key: i64) -> Self {

        Self {
            encrypted: data,
            private_key,
            decrypted: String::new(),
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


    pub fn decrypt(&mut self) {

        let mut output = Vec::new();

        for (c1, c2) in &self.encrypted {

            let shared = Self::scalar_mult(self.private_key, *c1);

            let inverse_shared = Point {
                x: shared.x,
                y: (P - shared.y) % P,
                infinity: shared.infinity,
            };

            let m_point = Self::point_add(*c2, inverse_shared);

            for i in 0..256 {
                if Self::scalar_mult(i, GENERATOR) == m_point {
                    output.push(i as u8);
                    break;
                }
            }
        } 

        self.decrypted = match String::from_utf8(output) {
            Ok(s) => s,
            Err(_) => String::new(),
        };
    }
}
