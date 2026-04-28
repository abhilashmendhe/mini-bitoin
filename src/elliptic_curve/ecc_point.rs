use std::ops::{Add, Mul};

use num_bigint::BigInt;

use crate::{
    elliptic_curve::curve_field::CurveField, finite_fields::field_element::FieldElement,
    utils::errors::BTCErr,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Point<T> {
    Infinite,
    Finite { x: T, y: T, a: T, b: T },
}

impl<T> Point<T>
where
    T: CurveField,
{
    pub fn new(x: T, y: T, a: T, b: T) -> Self {
        assert!(
            (y.clone() * y.clone())
                == (x.clone() * x.clone() * x.clone()) + (a.clone() * x.clone()) + b.clone()
        );
        Point::Finite { x, y, a, b }
    }
    pub fn try_new(x: T, y: T, a: T, b: T) -> Result<Self, BTCErr> {
        let checked_y_square = (y.clone().checked_mul(y.clone()))?;
        let checked_x_cube = ((x.clone().checked_mul(x.clone())?).checked_mul(x.clone()))?;
        let checked_a_x_mul = a.clone().checked_mul(x.clone())?;

        if checked_y_square
            != (checked_x_cube.checked_add(checked_a_x_mul)?).checked_add(b.clone())?
        {
            return Err(BTCErr::PointNotOnECC(format!(
                "({}, {}) is not on the curve",
                x, y
            )));
        }
        Ok(Point::Finite { x, y, a, b })
    }
    pub fn inifinity() -> Self {
        Point::Infinite
    }
    pub fn checked_add(self, rhs: Point<T>) -> Result<Self, BTCErr> {
        match (&self, &rhs) {
            (Point::Infinite, _) => Ok(rhs),
            (_, Point::Infinite) => Ok(self),
            (
                Point::Finite {
                    x: _,
                    y: _,
                    a: a1,
                    b: b1,
                },
                Point::Finite {
                    x: _,
                    y: _,
                    a: a2,
                    b: b2,
                },
            ) => {
                if a1 != a2 || b1 != b2 {
                    return Err(BTCErr::PointNotOnSameECC(format!(
                        "Points {}, {} are not on the same curve",
                        a1, b1
                    )));
                }

                Ok(self + rhs)
            }
        }
    }
    pub fn rmul(self, coeff: BigInt) -> Self {
        let mut coeff = coeff;
        let mut current = self.clone();
        let mut result = Point::Infinite::<T>;

        let one: BigInt = 1.into();
        while coeff > 0.into() {
            if (coeff.clone() & one.clone()) == one {
                result = result + current.clone();
            }
            current = current.clone() + current;
            coeff >>= 1;
        }
        result
    }
}

impl std::fmt::Display for Point<BigInt> {
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
            Point::Finite { a, b, x, y } => write!(
                f,
                "Point({},{})_{}_{} FieldElement({})",
                x.num, y.num, a.num, b.num, a.prime
            ),
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
                Point::Finite {
                    x: x1,
                    y: y1,
                    a: a1,
                    b: b1,
                },
                Point::Finite {
                    x: x2,
                    y: y2,
                    a: a2,
                    b: b2,
                },
            ) => {
                // check condition in checked add
                assert!(a1 == a2 && b1 == b2);
                if x1 == x2 && y1 != y2 {
                    return Point::inifinity();
                } else if x1 != x2 {
                    let slope = ((*y2).clone() - (*y1).clone()) / ((*x2).clone() - (*x1).clone());
                    let x3 = (slope.clone() * slope.clone()) - (*x1).clone() - (*x2).clone();
                    let y3 = slope * ((*x1).clone() - x3.clone()) - (*y1).clone();
                    return Point::Finite {
                        x: x3,
                        y: y3,
                        a: (*a1).clone(),
                        b: (*b1).clone(),
                    };
                }

                if self == rhs && *y1 == y1.zero() {
                    return Point::Infinite;
                }
                let x1_2 = (*x1).clone() * (*x1).clone();
                let x1_2_3 = x1_2.clone() + x1_2.clone() + x1_2.clone();
                let y1_1_2 = (*y1).clone() + (*y1).clone();
                let slope = (x1_2_3 + (*a1).clone()) / y1_1_2;
                let x3 = (slope.clone() * slope.clone()) - (*x1).clone() - (*x1).clone();
                let y3 = slope * ((*x1).clone() - x3.clone()) - (*y1).clone();
                return Point::Finite {
                    x: x3,
                    y: y3,
                    a: (*a1).clone(),
                    b: (*b1).clone(),
                };
            }
        }
    }
}

impl<T> Mul<BigInt> for Point<T>
where
    T: CurveField,
{
    type Output = Self;
    fn mul(self, rhs: BigInt) -> Self::Output {
        self.rmul(rhs)
    }
}
