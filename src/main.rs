extern crate stick;

use std::io::stdin;
//use stick::Input::Move;

fn main() {

    println!("Hello, world!");
    let stdin = stdin();
    let mut cm = stick::ControllerManager::new(vec![]);
//    let mut test1 = 0.00;
//    let mut test2 = 0.00;
    'a: loop {
        while let Some((j, i)) = cm.update(){

	println!("{}: {}", j, i);
//            match i {
//                        Move(x, y) => {
//			    test1 = y;
//			    test2 = x;
//                        }
//            }
        }
//    println!("x: {}, y: {}", test1, test2);
    }

}
