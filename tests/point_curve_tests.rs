use mini_bitoin::{elliptic_curve::ecc_point::Point, utils::errors::BTCErr};

fn print_point_isize(point: Result<Point<isize>, BTCErr>) {
    match point {
        Ok(p) => println!("✅ Success: Point on curve: {}", p),
        Err(err) => println!("❌ Error: {}",err),
    };
}

#[test]
fn point_init_test() {
    print_point_isize(Point::try_new(-1, -1, 5, 7));
    print_point_isize(Point::try_new(-1, -2, 5, 7));
    print_point_isize(Point::try_new(2, 4, 5,7));
    print_point_isize(Point::try_new(18, 77, 5,7));
    print_point_isize(Point::try_new(5, 7, 5,7));
}

#[test]
fn point_add_num_test() {
    let p1 = Point::new(-1, -1, 5, 7);
    let p2 = Point::new(-1, 1, 5, 7);
    let inf: Point<isize> = Point::inifinity();
    
    println!("p1 + inf = {}", p1.clone() + inf.clone());
    assert_eq!(p1.clone() + inf.clone(), p1.clone());

    println!("p2 + inf = {}", p2.clone() + inf.clone());
    assert_eq!(p2.clone(), inf.clone() + p2.clone());

    println!("p1 + p2 = {}", p1.clone() + p2.clone());
    assert_eq!(p1.clone() + p2.clone(), inf.clone());
}