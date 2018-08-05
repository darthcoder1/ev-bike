//#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;

use rt::ExceptionFrame;

entry!(main);

// These are the inputs coming from the driver controlls
struct Input
{
	// true when the ignition is enabled
	ignition : bool,
	// true when front brake is used
	brake_front : bool,
	// true when back brake is used
	brake_rear : bool,
	// true when the left turn signal is activated
	turn_left : bool,
	// true when the right turn signal is activated
	turn_right : bool,
	// true when the lights are turned on
	light_on : bool,
	// true when the full beam is turned on
	full_beam : bool,
	// true when horn button pressed
	horn : bool,
	// true when killswitch is on KILL
	kill_switch : bool,
	// true when sidestand is out
	side_stand : bool,
}


enum PowerChannel
{
	TurnLeftFront = 0,
	TurnLeftRear = 1,
	TurnRightFront = 2,
	TurnRightRear = 3,
	HeadLightParking = 4,
	HeadLightLowerBeam = 5,
	HeadLightFullBeam = 6,
	RearLight = 7,
	BrakeLight = 8,
	Horn=9,
}

fn read_input() -> Input {
	
	// TOOD: read input from pins

	let input = Input {
		ignition : true,
		brake_front : false,
		brake_rear : false,
		turn_left : false,
		turn_right : false,
		light_on : false,
		full_beam : false,
		horn : false, 
		kill_switch : false,
		side_stand : false,
	};

	input
}

fn operate_power_output(_input : &Input) {

}

fn sleep(seconds : f32)
{

}

fn main() -> ! {

    loop {

    	let input = read_input();
    	operate_power_output(&input);
    	sleep(0.05);
    }
 }

 // define the hard fault handler
 exception!(HardFault, hard_fault);

 fn hard_fault(ef: &ExceptionFrame) -> ! {
     panic!("HardFault at {:#?}", ef);
 }

 // define the default exception handler
 exception!(*, default_handler);

 fn default_handler(irqn: i16) {
     panic!("Unhandled exception (IRQn = {})", irqn);
 }