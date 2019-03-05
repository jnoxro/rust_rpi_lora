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

use rppal::uart::{Parity, Uart};

fn main() -> Result<(), Box<dyn Error>> {

    println!("running");

    let mut six_axis = joy::Device::open(b"/dev/input/js0\0").unwrap(); 
    let mut throt: i32 = 0;
    let mut throt_val: i32  = 0;
    let mut _sink = 0;

    let mut uart = Uart::new(9_600, Parity::None, 8, 1)?;
    uart.set_blocking_mode(1, Duration::default())?;
    let mut uart_buffer = [0u8; 1];

    let mut f : u8 = 0;
    let mut throt_u8 = [0u8; mem::size_of::<i32>()];


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
	throt_u8.as_mut().write_i32::<LittleEndian>(throt_val).expect("unable to convert");
//	f = 1000 * throt_u8[0] + 100 * throt_u8[1] + 10 * throt_u8[2] + throt_u8[3];
	f = throt_u8[0];
	uart.write(slice::from_mut(&mut f))?;

        println!("throttle: {}, sending: {}", throt_val, f);

	thread::sleep(time::Duration::from_millis(50));
    }

}
