use mini_bitoin::finite_fields::field_element::FieldElement;

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
