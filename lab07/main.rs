use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new<T: Into<f64>, S: Into<f64>>(f: T, s: S) -> Self {
        Complex {
            real: f.into(),
            imag: s.into(),
        }
    }
    fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }
}
impl<T> Add<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Complex;
    fn add(self, r: T) -> Self::Output {
        let c: Complex = r.into();
        return Complex {
            real: self.real + c.real,
            imag: self.imag + c.imag,
        };
    }
}
impl<T> Sub<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Complex;

    fn sub(self, r: T) -> Self::Output {
        let c = r.into();
        Complex {
            real: self.real - c.real,
            imag: self.imag - c.imag,
        }
    }
}

impl<T> Mul<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Complex;

    fn mul(self, r: T) -> Self::Output {
        let c = r.into();
        Complex {
            real: self.real * c.real - self.imag * c.imag,
            imag: self.real * c.imag + self.imag * c.real,
        }
    }
}
impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}
impl From<i32> for Complex {
    fn from(r: i32) -> Self {
        Complex {
            real: r as f64,
            imag: 0.0,
        }
    }
}

impl From<f64> for Complex {
    fn from(r: f64) -> Self {
        Complex { real: r, imag: 0.0 }
    }
}

impl PartialEq for Complex {
    fn eq(&self, o: &Self) -> bool {
        return (self.real == o.real) && (self.imag == o.imag);
    }
}

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.real == 0f64 {
            if self.imag == 0f64 {
                write!(f, "0",)?;
            } else {
                write!(f, "{:}i", self.imag)?;
            }
        } else {
            write!(f, "{}", self.real)?;
            if self.imag != 0f64 {
                write!(f, "{:+}i", self.imag)?;
            }
        }
        Ok(())
    }
}

macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}
