use crate::{elliptic_curve::curve_field::CurveField, finite_fields::field_element::FieldElement, utils::errors::BTCErr};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Point<T> {
    Infinite,
    Finite {
        a: T,
        b: T, 
        x: T, 
        y: T
    },
}

impl<T> Point<T>
    where 
        T: CurveField
{
    pub fn new(x: T, y: T, a: T, b: T) -> Self {
        assert!((y*y) == (x*x*x)+(a*x)+b);
        Point::Finite { a, b, x, y }
    }
    pub fn try_new(x: T, y: T, a: T, b: T) -> Result<Self, BTCErr> {
        let checked_y_square = (y.checked_mul(y))?;
        let checked_x_cube   = ((x.checked_mul(x)?).checked_mul(x))?;
        let checked_a_x_mul  = a.checked_mul(x)?;

        if checked_y_square != (checked_x_cube.checked_add(checked_a_x_mul)?).checked_add(b)? {
            return Err(BTCErr::PointNotOnECC(format!("({}, {}) is not on the curve", x, y)))
        }
        Ok(Point::Finite { a, b, x, y })
    }
    pub fn inifinity() -> Self {
        Point::Infinite
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