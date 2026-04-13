use mini_bitoin::{finite_fields::field_element::FieldElement, utils::errors::BTCErr};

#[test]
fn check_finite_field_range()  {
    match FieldElement::new(10, 7) {
        Ok(fe) => println!("✅ Success: Finite field: {}", fe),
        Err(err) => println!("❌ Error: {}",err),
    };
    
    println!();
    match FieldElement::new(12, 23) {
        Ok(fe) => println!("✅ Success: Finite field: {}", fe),
        Err(err) => println!("❌ Error: {}",err),
    };

    println!();
    match FieldElement::new(10, 13) {
        Ok(fe) => println!("✅ Success: Finite field: {}", fe),
        Err(err) => println!("❌ Error: {}",err),
    };

    println!();
    match FieldElement::new(-90, 107) {
        Ok(fe) => println!("✅ Success: Finite field: {}", fe),
        Err(err) => println!("❌ Error: {}",err),
    };

}



#[test]
fn compare_finite_field() -> Result<(), BTCErr> {
    let a = FieldElement::new(7, 13)?;
    let b = FieldElement::new(6, 13)?;
    assert!(a != b); 
    Ok(())
}

#[test]
fn add_finite_field() -> Result<(), BTCErr> {
    let a = FieldElement::new(7, 13)?;
    let b = FieldElement::new(12, 13)?;
    let c = FieldElement::new(6, 13)?;
    assert!((a+b)? == c);
    Ok(())
}

#[test]
fn sub_finite_field() -> Result<(), BTCErr> {
    let a = FieldElement::new(7, 13)?;
    let b = FieldElement::new(12, 13)?;
    let c = FieldElement::new(8, 13)?;
    assert!((a-b)? == c);
    Ok(())
}

#[test]
fn mul_finite_field() -> Result<(), BTCErr> {
    let a = FieldElement::new(3, 13)?;
    let b = FieldElement::new(12, 13)?;
    let c = FieldElement::new(10, 13)?;
    assert!((a*b)? == c);
    Ok(())
}

#[test]
fn exp_finite_field() -> Result<(), BTCErr> {
    let f1 = FieldElement::new(2, 5)?;
    let f2 = FieldElement::new(4, 5)?;
    assert!(f1.pow_modulo(10) == f2);

    let f3 = FieldElement::new(3, 23)?;
    let f4 = FieldElement::new(2, 23)?;
    assert!(f3.pow_modulo(29) == f4);

    Ok(())
}
