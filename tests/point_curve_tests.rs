use mini_bitoin::{elliptic_curve::ecc_point::Point, finite_fields::field_element::FieldElement, utils::errors::BTCErr};
use num_bigint::BigInt;

fn print_point_bigint(point: Result<Point<BigInt>, BTCErr>) {
    match point {
        Ok(p) => println!("✅ Success: Point on curve: {}", p),
        Err(err) => println!("❌ Error: {}",err),
    };
}
fn print_point_field_element(point: Result<Point<FieldElement>, BTCErr>) {
    match point {
        Ok(p) => println!("✅ Success: Point on curve: {}", p),
        Err(err) => println!("❌ Error: {}",err),
    };
}

#[test]
fn point_init_isize_test() {
    print_point_bigint(Point::try_new((-1).into(), (-1).into(), 5.into(), 7.into()));
    print_point_bigint(Point::try_new((-1).into(), (-2).into(), 5.into(), 7.into()));
    print_point_bigint(Point::try_new(2.into(), 4.into(), 5.into(),7.into()));
    print_point_bigint(Point::try_new(18.into(), 77.into(), 5.into(),7.into()));
    print_point_bigint(Point::try_new(5.into(), 7.into(), 5.into(),7.into()));
}

#[test]
fn point_init_field_element_test() {
    let a = FieldElement::new((0).into(), (103).into());
    let b = FieldElement::new((7).into(), (103).into());
    
    let x1 = FieldElement::new((17).into(), (103).into());
    let y1 = FieldElement::new((64).into(), (103).into());
    print_point_field_element(Point::try_new(x1, y1, a, b));

    let a = FieldElement::new((0).into(), (223).into());
    let b = FieldElement::new((7).into(), (223).into());
    
    let x1 = FieldElement::new((192).into(), (223).into());
    let y1 = FieldElement::new((105).into(), (223).into());
    print_point_field_element(Point::try_new(x1, y1, a.clone(), b.clone()));

    let x1 = FieldElement::new((17).into(), (223).into());
    let y1 = FieldElement::new((56).into(), (223).into());
    print_point_field_element(Point::try_new(x1, y1, a.clone(), b.clone()));

    let x1 = FieldElement::new((200).into(), (223).into());
    let y1 = FieldElement::new((119).into(), (223).into());
    print_point_field_element(Point::try_new(x1, y1, a.clone(), b.clone()));

    let x1 = FieldElement::new((1).into(), (223).into());
    let y1 = FieldElement::new((193).into(), (223).into());
    print_point_field_element(Point::try_new(x1, y1, a.clone(), b.clone()));

    let x1 = FieldElement::new((42).into(), (223).into());
    let y1 = FieldElement::new((99).into(), (223).into());
    print_point_field_element(Point::try_new(x1, y1, a.clone(), b.clone()));
}

#[test]
fn point_add_field_elem_test() {
    let prime: BigInt = (223).into();
    let a = FieldElement::new((0).into(), prime.clone());
    let b = FieldElement::new((7).into(), prime.clone());

    // Test (1).into()
    let x1 = FieldElement::new((192).into(), prime.clone());
    let y1 = FieldElement::new((105).into(), prime.clone());
    let x2 = FieldElement::new((17).into(), prime.clone());
    let y2 = FieldElement::new((56).into(), prime.clone());
    let p1 = Point::new(x1, y1, a.clone(), b.clone());
    let p2 = Point::new(x2, y2, a.clone(), b.clone());
    let p3 = Point::new(FieldElement::new((170).into(), prime.clone()), FieldElement::new((142).into(), prime.clone()), a.clone(), b.clone());
    println!("p1 + p2 = {}", p1.clone() + p2.clone());
    assert_eq!(p1 + p2, p3);

    // Test (2).into()
    let x1 = FieldElement::new((170).into(), prime.clone());
    let y1 = FieldElement::new((142).into(), prime.clone());
    let x2 = FieldElement::new((60).into(),  prime.clone());
    let y2 = FieldElement::new((139).into(), prime.clone());
    let p1 = Point::new(x1, y1, a.clone(), b.clone());
    let p2 = Point::new(x2, y2, a.clone(), b.clone());
    let p3 = Point::new(FieldElement::new((220).into(), prime.clone()), FieldElement::new((181).into(), prime.clone()), a.clone(), b.clone());
    println!("p1 + p2 = {}", p1.clone() + p2.clone());
    assert_eq!(p1 + p2, p3);

    // Test (3).into()
    let x1 = FieldElement::new((47).into(), prime.clone());
    let y1 = FieldElement::new((71).into(), prime.clone());
    let x2 = FieldElement::new((17).into(), prime.clone());
    let y2 = FieldElement::new((56).into(), prime.clone());
    let p1 = Point::new(x1, y1, a.clone(), b.clone());
    let p2 = Point::new(x2, y2, a.clone(), b.clone());
    let p3 = Point::new(FieldElement::new((215).into(), prime.clone()), FieldElement::new((68).into(), prime.clone()), a.clone(), b.clone());
    println!("p1 + p2 = {}", p1.clone() + p2.clone());
    assert_eq!(p1 + p2, p3);

    // Test (4).into()
    let x1 = FieldElement::new((143).into(), prime.clone());
    let y1 = FieldElement::new((98).into(),  prime.clone());
    let x2 = FieldElement::new((76).into(),  prime.clone());
    let y2 = FieldElement::new((66).into(),  prime.clone());
    let p1 = Point::new(x1, y1, a.clone(), b.clone());
    let p2 = Point::new(x2, y2, a.clone(), b.clone());
    let p3 = Point::new(FieldElement::new((47).into(), prime.clone()), FieldElement::new((71).into(), prime.clone()), a.clone(), b.clone());
    println!("p1 + p2 = {}", p1.clone() + p2.clone());
    assert_eq!(p1 + p2, p3);
}