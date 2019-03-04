extern crate joy;
//extern crate null_terminated;

fn main() {

    println!("running");

    let mut six_axis = joy::Device::open(b"/dev/input/js0\0").unwrap(); 
    let mut throt: i32 = 0;
    let mut throt_val: i32  = 0;
    let mut _sink = 0;

      loop {
        for ev in &mut six_axis {

	    use joy::Event::*;
	    match ev {
		Axis(5, x) => throt = x.into(),
	        Axis(_n, _x) => _sink = 1,
		Button(n, true) => _sink = 1,
		Button(n, false) => _sink = 1,
	    }
//	    throt = throt + 32767;
//	    throt = throt * 100;
//	    throt = throt / (32767*2);
        }
	throt_val = throt + 32767;
	throt_val = throt_val * 100;
	throt_val = throt_val / (32767*2);

        println!("throttle: {}", throt_val);
      }

}
