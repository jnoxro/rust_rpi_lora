//author: Jack Orton
//Date: 5th March 19
//program takes the input from a gamepad, joystick js0 // /dev/input/js0  (in this case a ps3 controller connected via bluetooth)
//outputs this value over uart serial to a LoRa module for long range transmittion


extern crate joy;
extern crate rppal;
extern crate byteorder;

use byteorder::{LittleEndian, WriteBytesExt};
use std::slice;
use std::error::Error;
use std::time::Duration;
use std::time;
use std::thread;
use std::mem;
use std::string::String;

use rppal::uart::{Parity, Uart};

fn main() -> Result<(), Box<dyn Error>> {

    println!("running");

    let mut six_axis = joy::Device::open(b"/dev/input/js0\0").unwrap(); 

    let mut throtl: i32 = 0;
    let mut throtl_val: i32  = 0;
    let mut throtr: i32 = 0;    
    let mut throtr_val: i32 = 0;

    let mut _sink = 0;

    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;
    uart.set_blocking_mode(1, Duration::default())?;
    let mut uart_buffer = [0u8; 1];

    let mut f : u8 = 0;
    let mut throtl_u8 = [0u8; mem::size_of::<i32>()];
    let mut throtr_u8 = [0u8; mem::size_of::<i32>()];
    let mut output = String::from("");

    loop {

        //Get values from controller
        for ev in &mut six_axis {

  	    use joy::Event::*;
	    match ev {
	        Axis(5, x) => throtr = x.into(),
		Axis(2, x) => throtl = x.into(),
	        Axis(_n, _x) => _sink = 1,
	        Button(_n, true) => _sink = 1,
	        Button(_n, false) => _sink = 1,
	    }
        }

	//convert values
        throtl_val = throtl + 32767;
        throtl_val = throtl_val * 100 * 255;
        throtl_val = throtl_val / (3276700*2);

	throtr_val = throtr + 32767;
	throtr_val = throtr_val * 100 * 255;
	throtr_val = throtr_val / (3276700*2);
	

//	if uart.read(&mut uart_buffer)? > 0 {
//	    println!("byte in: {}", uart_buffer[0]);
//            f = uart_buffer[0].into();
//	}
	
//	f = f * 2;
//        println!("yaboi f: {}", f);


	throtl_u8.as_mut().write_i32::<LittleEndian>(throtl_val).expect("unable to convert");
	throtr_u8.as_mut().write_i32::<LittleEndian>(throtr_val).expect("unable to convert");
	

//	f = throt_u8[0];
	f = 0;
	output.push_str(&throtl_u8[0].to_string());
	output.push('l');
	output.push_str(&throtr_u8[0].to_string());
	output.push('r');
        println!("throtl: {}, throtr: {},  sending: {}", throtl_val, throtr_val,  output);

//	uart.write(slice::from_mut(&mut f))?;
	uart.write(output.as_bytes())?;



	output = "".to_string();
	thread::sleep(time::Duration::from_millis(70));
    }

}
