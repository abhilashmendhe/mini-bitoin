use mini_bitoin::{finite_fields::field_element::FieldElement, utils::errors::BTCErr};

#[test]
fn check_finite_field_range()  {
    match FieldElement::try_new(10, 7) {
        Ok(fe) => println!("✅ Success: Finite field: {}", fe),
        Err(err) => println!("❌ Error: {}",err),
    };
    
    println!();
    match FieldElement::try_new(12, 23) {
        Ok(fe) => println!("✅ Success: Finite field: {}", fe),
        Err(err) => println!("❌ Error: {}",err),
    };

    println!();
    match FieldElement::try_new(10, 13) {
        Ok(fe) => println!("✅ Success: Finite field: {}", fe),
        Err(err) => println!("❌ Error: {}",err),
    };

    println!();
    match FieldElement::try_new(-90, 107) {
        Ok(fe) => println!("✅ Success: Finite field: {}", fe),
        Err(err) => println!("❌ Error: {}",err),
    };

}



#[test]
fn compare_finite_field() -> Result<(), BTCErr> {
    let a = FieldElement::try_new(7, 13)?;
    let b = FieldElement::try_new(6, 13)?;
    assert!(a != b); 
    Ok(())
}

#[test]
fn add_finite_field() -> Result<(), BTCErr> {
    let a = FieldElement::try_new(7, 13)?;
    let b = FieldElement::try_new(12, 13)?;
    let c = FieldElement::try_new(6, 13)?;
    assert!(a.checked_add(b)? == c);
    Ok(())
}

#[test]
fn sub_finite_field() -> Result<(), BTCErr> {
    let a = FieldElement::try_new(7, 13)?;
    let b = FieldElement::try_new(12, 13)?;
    let c = FieldElement::try_new(8, 13)?;
    println!("{}",a.checked_sub(b)?);
    assert!(a.checked_sub(b)? == c);
    Ok(())
}

#[test]
fn mul_finite_field() -> Result<(), BTCErr> {
    let a = FieldElement::try_new(3, 13)?;
    let b = FieldElement::try_new(12, 13)?;
    let c = FieldElement::try_new(10, 13)?;
    assert!(a.checked_mul(b)? == c);
    Ok(())
}

#[test]
fn exp_finite_field() -> Result<(), BTCErr> {
    let f1 = FieldElement::try_new(2, 5)?;
    let f2 = FieldElement::try_new(4, 5)?;
    assert!(f1.pow_modulo(10) == f2);

    let f3 = FieldElement::try_new(3, 23)?;
    let f4 = FieldElement::try_new(2, 23)?;
    assert!(f3.pow_modulo(29) == f4);
    Ok(())
}


#[test]
fn div_finite_field() -> Result<(), BTCErr> {
    let f1 = FieldElement::try_new(2, 19)?;
    let f2 = FieldElement::try_new(7, 19)?;
    let f3 = FieldElement::try_new(3, 19)?;
    assert!((f1/f2) == f3);

    let f1 = FieldElement::try_new(7, 19)?;
    let f2 = FieldElement::try_new(5, 19)?;
    let f3 = FieldElement::try_new(9, 19)?;
    assert!((f1/f2) == f3);
    Ok(())
}

#[test]
fn neg_exp_finite_field() -> Result<(), BTCErr> {
    let f1 = FieldElement::try_new(7, 13)?;
    let f2 = FieldElement::try_new(8, 13)?;
    assert!(f1.pow_modulo(-3) == f2);    
    Ok(())
}