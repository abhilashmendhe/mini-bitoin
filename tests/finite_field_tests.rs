use mini_bitoin::{finite_fields::field_element::FieldElement, utils::errors::BTCErr};

#[test]
fn check_finite_field_range()  {
    match FieldElement::try_new(10.into(), 7.into()) {
        Ok(fe) => println!("✅ Success: Finite field: {}", fe),
        Err(err) => println!("❌ Error: {}",err),
    };
    
    println!();
    match FieldElement::try_new(12.into(), 23.into()) {
        Ok(fe) => println!("✅ Success: Finite field: {}", fe),
        Err(err) => println!("❌ Error: {}",err),
    };

    println!();
    match FieldElement::try_new(10.into(), 13.into()) {
        Ok(fe) => println!("✅ Success: Finite field: {}", fe),
        Err(err) => println!("❌ Error: {}",err),
    };

    println!();
    match FieldElement::try_new((-90).into(), 107.into()) {
        Ok(fe) => println!("✅ Success: Finite field: {}", fe),
        Err(err) => println!("❌ Error: {}",err),
    };

}

#[test]
fn compare_finite_field() -> Result<(), BTCErr> {
    let a = FieldElement::try_new(7.into(), 13.into())?;
    let b = FieldElement::try_new(6.into(), 13.into())?;
    assert!(a != b); 
    Ok(())
}

#[test]
fn add_finite_field() -> Result<(), BTCErr> {
    let a = FieldElement::try_new(7.into(), 13.into())?;
    let b = FieldElement::try_new(12.into(), 13.into())?;
    let c = FieldElement::try_new(6.into(), 13.into())?;
    assert!(a.checked_add(b)? == c);
    Ok(())
}

#[test]
fn sub_finite_field() -> Result<(), BTCErr> {
    let a = FieldElement::try_new(7.into(), 13.into())?;
    let b = FieldElement::try_new(12.into(), 13.into())?;
    let c = FieldElement::try_new(8.into(), 13.into())?;
    assert!(a.checked_sub(b)? == c);
    Ok(())
}

#[test]
fn mul_finite_field() -> Result<(), BTCErr> {
    let a = FieldElement::try_new(3.into(), 13.into())?;
    let b = FieldElement::try_new(12.into(), 13.into())?;
    let c = FieldElement::try_new(10.into(), 13.into())?;
    assert!(a.checked_mul(b)? == c);
    Ok(())
}

#[test]
fn exp_finite_field() -> Result<(), BTCErr> {
    let f1 = FieldElement::try_new(2.into(), 5.into())?;
    let f2 = FieldElement::try_new(4.into(), 5.into())?;
    assert!(f1.pow_modulo(10.into()) == f2);

    let f3 = FieldElement::try_new(3.into(), 23.into())?;
    let f4 = FieldElement::try_new(2.into(), 23.into())?;
    assert!(f3.pow_modulo(29.into()) == f4);
    Ok(())
}


#[test]
fn div_finite_field() -> Result<(), BTCErr> {
    let f1 = FieldElement::try_new(2.into(), 19.into())?;
    let f2 = FieldElement::try_new(7.into(), 19.into())?;
    let f3 = FieldElement::try_new(3.into(), 19.into())?;
    assert!((f1/f2) == f3);

    let f1 = FieldElement::try_new(7.into(), 19.into())?;
    let f2 = FieldElement::try_new(5.into(), 19.into())?;
    let f3 = FieldElement::try_new(9.into(), 19.into())?;
    assert!((f1/f2) == f3);
    Ok(())
}

#[test]
fn neg_exp_finite_field() -> Result<(), BTCErr> {
    let f1 = FieldElement::try_new(7.into(), 13.into())?;
    let f2 = FieldElement::try_new(8.into(), 13.into())?;
    assert!(f1.pow_modulo((-3).into()) == f2);    
    Ok(())
}