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

use rand::{Rng};
use rand::rngs::ThreadRng;

pub fn create_keys()->f64{
	let mut rng: ThreadRng= rand::thread_rng();
	let y: f64 = rng.gen(); 
  y
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