// fn create_keys()->String{
// 	String::from("iamaprivatekey.ipromise")
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_keys() {
//    		let key = create_keys();
//       println!("{}",key)
//     }
    
// }


// -----------------------
// use rand;

// pub fn create_keys()->f64{
// 	let mut rng = rand::thread_rng();
// 	let y: f64 = rng.gen(); 
//   y
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_keys() {
//    		let key = create_keys();
//       println!("{}",key)
//     }
    
// }

// -------------------

// use rand::Rng;

// pub fn create_keys()->f64{
// 	let mut rng = rand::thread_rng();
// 	let y: f64 = rng.gen(); 
//   y
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_keys() {
//    		let key = create_keys();
//       println!("{}",key)
//     }
    
// }

// -------------------


// use rand::{thread_rng, Rng};

// pub fn create_keys()->f64{
// 	let mut rng:thread_rng = rand::thread_rng();
// 	let y: f64 = rng.gen(); 
//   y
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_keys() {
//    		let key = create_keys();
//       println!("{}",key)
//     }
    
// }


// ------

// use rand::{Rng};
// use rand::rngs::ThreadRng;

// pub fn create_keys()->f64{
// 	let mut rng: ThreadRng= rand::thread_rng();
// 	let y: f64 = rng.gen(); 
//   y
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_keys() {
//    		let key = create_keys();
//       println!("{}",key)
//     }
    
// }
// ---------------
// use rand::{thread_rng, Rng};
// use rand::rngs::ThreadRng;
// use secp256k1::rand::rngs::OsRng;
// use secp256k1::{Secp256k1, Message};
// use secp256k1::bitcoin_hashes::sha256;



// pub fn create_keys()->String{
//   let secp = Secp256k1::new();
//   let mut rng: ThreadRng= rand::thread_rng();
//   let (secret_key, public_key) = secp.generate_keypair(&mut rng);
//   secret_key.to_string()
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_keys() {
//    		let key = create_keys();
//       println!("{}",key)
//     }
    
// }


// ----------

// use rand::{thread_rng, Rng};
// use rand::rngs::ThreadRng;
// use secp256k1::rand::rngs::OsRng;
// use secp256k1::{Secp256k1, Message};



// pub fn create_keys()->String{
// 	let secp = Secp256k1::new();
//   let mut rng: ThreadRng= rand::thread_rng();
//   let (secret_key, public_key) = secp.generate_keypair(&mut rng);
//   secret_key.to_string()
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_keys() {
//    		let key = create_keys();
//       println!("{}",key)
//     }
    
// }


// ---

// use rand::{thread_rng, Rng};
// // use rand::rngs::ThreadRng;
// use secp256k1::rand::rngs::OsRng;
// use secp256k1::{Secp256k1, Message};



// pub fn create_keys()->String{
// 	let secp = Secp256k1::new();
//   let mut rng = OsRng::new().expect("OsRng");
//   let (secret_key, public_key) = secp.generate_keypair(&mut rng);
//   secret_key.to_string()
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_keys() {
//    		let key = create_keys();
//       println!("{}",key)
//     }
    
// }


// -----

// use rand::{thread_rng, Rng};
// // use rand::rngs::ThreadRng;
// use secp256k1::rand::rngs::OsRng;
// use secp256k1::{Secp256k1, Message};


// pub fn create_keys()->Result<String,String>{
// 	let secp = Secp256k1::new();
  
//   let mut rng: OsRng = match OsRng::new(){
//   	Err(_)=> return Err(String::from("OSRng Error")),
//     Ok(rng)=>rng
//   };
  
//   let (secret_key, public_key) = secp.generate_keypair(&mut rng);
//   Ok(secret_key.to_string())
// }



// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn test_create_keys() {
// //    		let key = create_keys().unwrap();
// //       println!("{}",key)
// //     }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_keys() {
//    		let key = match create_keys(){
//       		Err(e)=> e,
//     			Ok(key)=>key
//       };
//       println!("{}",key);
//       assert_eq!(64,key.len());
//     }        
// }


// ---------------------



// https://docs.rs/rand/0.8.3/rand/rngs/struct.ThreadRng.html
use rand::{thread_rng, Rng};
// use rand::rngs::ThreadRng;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, Message, SecretKey};
use secp256k1::bitcoin_hashes::sha256;
use std::str::FromStr;

pub fn create_keys()->Result<String,String>{
	let secp = Secp256k1::new();
  
  let mut rng: OsRng = match OsRng::new(){
  	Err(_)=> return Err(String::from("OSRng Error")),
    Ok(rng)=>rng
  };
  
  let (secret_key, public_key) = secp.generate_keypair(&mut rng);
  Ok(secret_key.to_string())
  
}

pub fn sign(priv_key:&str, message:&str)->Result<String, String>{
  let secp = Secp256k1::new();
	let msg = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
  let secret =  SecretKey::from_str(priv_key).unwrap();
	let sig = secp.sign(&msg, &secret);
	
  Ok(sig.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_keys() {
   		let key = match create_keys(){
      		Err(e)=> e,
    			Ok(key)=>key
      };
      println!("{}",key);
      assert_eq!(64,key.len())
    }

    #[test]
    fn test_sign() {
   		let signature = match sign("8cae5405272acb5ff5669a03c88265cee1c6f3ad56350414ac889e7b51f15e05", "Jababab"){
      		Err(e)=> e,
    			Ok(key)=>key
      };
      println!("{}",signature);
      let expected_signature = "3045022100f7aa0e3cf293a170bf3889e1f47c190470d6c499632f43daab78223be6c3721202201e9f35a8175456956c995b20e7e3850adfb7e566a371b849ea1575939fac2040";
      assert_eq!(expected_signature, signature)
    }
}