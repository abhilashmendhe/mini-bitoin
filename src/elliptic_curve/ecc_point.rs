use std::ops::Add;

use crate::{elliptic_curve::curve_field::CurveField, finite_fields::field_element::FieldElement, utils::errors::BTCErr};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Point<T> {
    Infinite,
    Finite {
        x: T,
        y: T, 
        a: T, 
        b: T
    },
}

impl<T> Point<T>
    where 
        T: CurveField
{
    pub fn new(x: T, y: T, a: T, b: T) -> Self {
        assert!((y*y) == (x*x*x)+(a*x)+b);
        Point::Finite { x, y, a, b }
    }
    pub fn try_new(x: T, y: T, a: T, b: T) -> Result<Self, BTCErr> {
        let checked_y_square = (y.checked_mul(y))?;
        let checked_x_cube   = ((x.checked_mul(x)?).checked_mul(x))?;
        let checked_a_x_mul  = a.checked_mul(x)?;

        if checked_y_square != (checked_x_cube.checked_add(checked_a_x_mul)?).checked_add(b)? {
            return Err(BTCErr::PointNotOnECC(format!("({}, {}) is not on the curve", x, y)))
        }
        Ok(Point::Finite { x, y, a, b })
    }
    pub fn inifinity() -> Self {
        Point::Infinite
    }
    pub fn checked_add(self, rhs: Point<T>) {
        
    }
}

impl std::fmt::Display for Point<isize> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Point::Infinite => write!(f, "Point(infinity)"),
            Point::Finite { a, b, x, y } => write!(f, "Point({},{})_{}_{}", x, y, a, b),
        }
    }
}

impl std::fmt::Display for Point<FieldElement> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Point::Infinite => write!(f, "Point(infinity)"),
            Point::Finite { a, b, x, y } => 
            write!(f, "Point({},{})_{}_{} FieldElement({})", x.num, y.num, a.num, b.num, a.prime),
        }
    }
}

/* 
    // use std::any::{Any, TypeId};

    // fn is_field<T: 'static>(_: &T) {
    //     if TypeId::of::<T>() == TypeId::of::<FieldElement>() {
    //         println!("FieldElement!");
    //     }
    // }
*/

impl<T: CurveField> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        
        match (&self, &rhs) {
            (Point::Infinite, _) => rhs,
            (_, Point::Infinite) => self,
            (
                Point::Finite { x: x1, y: y1, a: a1, b: b1 },
                Point::Finite { x: x2, y: y2, a: a2, b: b2  },
            ) => {
                // check condition in checked add
                assert!(a1 == a2 && b1 == b2);
                if x1 == x2 && y1 != y2 {
                    return Point::inifinity();
                } else if x1 != x2 {
                    let slope = (*y2 - *y1) / (*x2 - *x1);
                    let x3 = (slope * slope) - *x1 - *x2;
                    let y3 = slope * (*x1 - x3) - *y1;
                    return Point::Finite { x: x3, y: y3, a: *a1, b: *b1 };
                }
               
                if self == rhs && *y1 == y1.zero() {
                    return Point::Infinite;
                }
                let x1_2 = (*x1) * (*x1);
                let x1_2_3 = x1_2 + x1_2 + x1_2;
                let y1_1_2 = *y1 + *y1;
                let slope = (x1_2_3 + *a1) / y1_1_2;
                let x3 = (slope * slope) - *x1 - *x1;
                let y3 = slope * (*x1 - x3) - *y1;
                return Point::Finite { x: x3, y: y3, a: *a1, b: *b1 };
            }
        }
    }
}