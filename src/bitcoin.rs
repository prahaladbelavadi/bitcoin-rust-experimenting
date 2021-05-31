fn create_keys()->String{
	String::from("iamaprivatekey.ipromise")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_keys() {
   		let key = create_keys();
      println!("{}",key)
    }
    
}