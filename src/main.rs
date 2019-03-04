extern crate joy;
extern crate rppal;

use std::slice;
use std::error::Error;
use std::time::Duration;

use rppal::uart::{Parity, Uart};

fn main() -> Result<(), Box<dyn Error>> {

    println!("running");

    let mut six_axis = joy::Device::open(b"/dev/input/js0\0").unwrap(); 
    let mut throt: i32 = 0;
    let mut throt_val: i32  = 0;
    let mut _sink = 0;

    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;
    uart.set_blocking_mode(1, Duration::default())?;
    let mut uart_buffer = [0u8; 1];

    let mut f : u8 = 0;

    loop {

        //Get values from controller
        for ev in &mut six_axis {

  	    use joy::Event::*;
	    match ev {
	        Axis(5, x) => throt = x.into(),
	        Axis(_n, _x) => _sink = 1,
	        Button(_n, true) => _sink = 1,
	        Button(_n, false) => _sink = 1,
	    }
        }

	//convert values
        throt_val = throt + 32767;
        throt_val = throt_val * 100;
        throt_val = throt_val / (32767*2);
	

//	if uart.read(&mut uart_buffer)? > 0 {
//	    println!("byte in: {}", uart_buffer[0]);
//            f = uart_buffer[0].into();
//	}
	
//	f = f * 2;
//        println!("yaboi f: {}", f);
	
	uart.write(slice::from_mut(&mut throt_val))?;

        println!("throttle: {}", throt_val);
    }

}
