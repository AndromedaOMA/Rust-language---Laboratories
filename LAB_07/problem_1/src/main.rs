use std::fmt;

//source: https://doc.rust-lang.org/std/ops/trait.Add.html
#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    r: f64,
    i: f64,
}
//======================================================================== DONE
//source: https://doc.rust-lang.org/std/convert/trait.From.html
impl From<i32> for Complex {
    fn from(value: i32) -> Self {
        Complex {
            r: value as f64,
            i: 0.0,
        }
    }
}
impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Complex { r: value, i: 0.0 }
    }
}
//======================================================================== DONE
//source: https://www.mathsisfun.com/algebra/complex-number-multiply.html
//source: https://doc.rust-lang.org/std/ops/trait.Add.html
use std::ops::Add;
impl<T> Add<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Complex;

    fn add(self, other: T) -> Complex {
        let other = other.into();
        Complex {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}
//source: https://doc.rust-lang.org/std/ops/trait.Sub.html
use std::ops::Sub;
impl<T> Sub<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Complex;

    fn sub(self, other: T) -> Complex {
        let other = other.into();
        Complex {
            r: self.r - other.r,
            i: self.i - other.i,
        }
    }
}
//source: https://doc.rust-lang.org/std/ops/trait.Mul.html
use std::ops::Mul;
impl<T> Mul<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Complex;

    fn mul(self, other: T) -> Complex {
        let other = other.into();
        Complex {
            r: self.r * other.r - self.i * other.i,
            i: self.r * other.i + self.i * other.r,
        }
    }
}
//======================================================================== DONE
//source: https://doc.rust-lang.org/std/ops/trait.Neg.html
use std::ops::Neg;
impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex {
            r: (self.r * -1.0),
            i: (self.i * -1.0),
        }
    }
}
//======================================================================== DONE
trait ComplexMethods {
    fn new<T1, T2>(value1: T1, value2: T2) -> Self
    where
        T1: Into<f64>,
        T2: Into<f64>;
    fn conjugate(&self) -> Complex;
}
impl ComplexMethods for Complex {
    fn new<T1, T2>(value1: T1, value2: T2) -> Self
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        Complex {
            r: value1.into(),
            i: value2.into(),
        }
    }

    fn conjugate(&self) -> Complex {
        Complex {
            r: self.r,
            i: (self.i * (-1.0)),
        }
    }
}
//======================================================================== DONE
//source: https://doc.rust-lang.org/std/fmt/trait.Display.html
use std::fmt::Display;
impl Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.r == 0.0 && self.i == 0.0 {
            write!(f, "0")
        } else if self.i == 0.0 {
            write!(f, "{}", self.r)
        } else if self.r == 0.0 {
            write!(f, "{}i", self.i)
        } else if self.i < 0.0 {
            write!(f, "{}{}i", self.r, self.i)
        } else {
            write!(f, "{}+{}i", self.r, self.i)
        }
        //--PRIMESC ERORI SI NU STIU SA LE REZOLV...
        // match (&self.r, self.i) {
        //     (r, 0.0) => write!(f, "{}", self.r),
        //     (r, i) if i > 0.0 => write!(f, "{}+{}i", self.r, self.i),
        //     (0, i) => write!(f, "{}i", self.i),
        //     (r, i) => write!(f, "{}{}i", self.r, self.i),
        // }
    }
}

//========================================================================
//========================================================================

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
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
    assert_eq_rel!(a.r, 1);
    assert_eq_rel!(a.i, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.r, 3);
    assert_eq_rel!(c.i, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.i, -7);

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
    assert_eq_rel!(i.r, -6);

    let j = -i + i;
    assert_eq_rel!(j.r, 0);
    assert_eq_rel!(j.i, 0);

    println!("ok!");
}
