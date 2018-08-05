//#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
#[macro_use]
//extern crate stm32f103xx;
extern crate stm32f103xx_hal as hal;
#[macro_use(block)]
extern crate nb;

use rt::ExceptionFrame;

use cortex_m::peripheral::syst::SystClkSource;
use stm32f103xx::Interrupt;
use hal::prelude::*;
use hal::stm32f103xx;
use hal::timer::Timer;

entry!(main);

// Main entry
fn main() -> ! {

	let _cortex_m = cortex_m::Peripherals::take().unwrap();
	let _stm32f103 = stm32f103xx::Peripherals::take().unwrap();

	let mut _flash = _stm32f103.FLASH.constrain();
	let mut _rcc = _stm32f103.RCC.constrain();

	let clocks = _rcc.cfgr.freeze(& mut _flash.acr);


	let mut gpioc = _stm32f103.GPIOC.split(& mut _rcc.apb2);
	let mut led = gpioc.pc13.into_push_pull_output(& mut gpioc.crh);

	let mut timer = Timer::syst(_cortex_m.SYST, 1.hz(), clocks);

	// setup the turn signal state
	let mut _system_state = System {
		turn_left : State::Inactive,
		turn_right : State::Inactive,
		hazard : State::Inactive,
	};

    loop {
    	let _input = read_input();

    	// switch power
		_system_state = update_system_state(_system_state, &_input);
    	let _power_out = switch_power_output(&_system_state, &_input);

    	// TEST
    	block!(timer.wait()).unwrap();
    	led.set_high();
    	block!(timer.wait()).unwrap();
    	led.set_low();
    	// apply power state to hardware

    	// read diagnosis from PFETs

    	// output telemetry data
    }
 }




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
	// true when hazard light is turned on
	hazard_light : bool,
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

// This will be the translation for the harware channel number
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

// This describes how the power ouptput needs to be swithed, which
// outputs to open and which to close
struct PowerOutput
{
	turn_left_front : bool,
	turn_left_rear : bool,
	turn_right_front : bool,
	turn_right_rear : bool,
	head_light_parking : bool,
	head_light_lowbeam : bool,
	head_light_fullbeam : bool,
	rear_light : bool,
	brake_light : bool,
	horn : bool,
}

// TODO. This will read the input data pins from the driver controlls
// and fill in the Input structure
fn read_input() -> Input {
	
	// TOOD: read input from pins

	let input = Input {
		ignition : true,
		brake_front : false,
		brake_rear : false,
		turn_left : false,
		turn_right : false,
		hazard_light : false,
		light_on : false,
		full_beam : false,
		horn : false, 
		kill_switch : false,
		side_stand : false,
	};

	input
}

#[derive(Clone, Copy)]
struct TimeStamp(i32);

// TODO: get time from device
fn device_get_ticks() -> TimeStamp {
	
	TimeStamp(0)
}

// TODO: convert ms to ticks
fn time_milliseconds_to_ticks(ms : i32) -> i32 {
	ms
}

// This indicates s state and if active, since which tick
enum State
{
	Active(TimeStamp),
	Inactive,
}

// This is the system state that needs to be kept around for timing relevant
// state
struct System
{
	turn_left : State,
	turn_right : State,
	hazard : State,
}


fn update_state(_prev_state : State, _input_flag : bool) -> State {
	
	if _input_flag  {
		match _prev_state {
			State::Active(time) => _prev_state,
			State::Inactive => State::Active(device_get_ticks()),
		}
	}
	else {
		State::Inactive
	}
}

fn update_system_state(_system : System, _input : &Input) -> System {

	System {
		turn_left : update_state(_system.turn_left, _input.turn_left),
		turn_right : update_state(_system.turn_right, _input.turn_right),
		hazard : update_state(_system.hazard, _input.hazard_light),
	}
}

fn caclulate_turn_signal(_state : &State, _cur_time : TimeStamp, _on_time : i32, _off_time : i32) -> bool {

	match _state {
		State::Active(start_time) => {
			let _time_passed = _cur_time.0 - start_time.0;
			let _passed_cycles_mod = _time_passed % (_on_time + _off_time);

			if _passed_cycles_mod < _on_time {
				true
			}
			else {
				false
			}
		},
		State::Inactive => false,
	}
}

// Switch the turn signals based on the driver input and also calculate the signaling based on the 
// defined turn signal interval.
//
// Order of evaluations:
//  * Hazard:		If hazard is activated, turn signal input is ignored, and all turn signal lights blink with
//					the defined frequency
//  * Turn_Left/
//	  Turn_Right:	If turn signal is activated, the turn signal lights on the according side will blink
//					with the defined frequency
fn switch_turn_signals(_system_state : &System, _input : &Input, _power_out : & mut PowerOutput)
{
	let current_time = device_get_ticks();
	let _one_sec_in_ticks = time_milliseconds_to_ticks(1000);

	let _hazard_on = match _system_state.hazard {
		State::Active(start_time) => (true, caclulate_turn_signal(&_system_state.hazard, current_time, _one_sec_in_ticks, _one_sec_in_ticks)),
		State::Inactive => (false, false),
	};
	
	let _turn_left_on = match _system_state.turn_left {
		State::Active(start_time) => (true, caclulate_turn_signal(&_system_state.turn_left, current_time, _one_sec_in_ticks, _one_sec_in_ticks)),
		State::Inactive => (false, false),
	};
	
	let _turn_right_on = match _system_state.turn_right {
		State::Active(start_time) => (true, caclulate_turn_signal(&_system_state.turn_right, current_time, _one_sec_in_ticks, _one_sec_in_ticks)),
		State::Inactive => (false, false),
	};

	if _hazard_on.0 {
		_power_out.turn_right_front = _hazard_on.1;
		_power_out.turn_right_rear = _hazard_on.1;
		_power_out.turn_left_front = _hazard_on.1;
		_power_out.turn_left_rear = _hazard_on.1;
	}
	else {
		if _turn_left_on.0 {
			_power_out.turn_left_front = _turn_left_on.1;
			_power_out.turn_left_rear = _turn_left_on.1;
			_power_out.turn_right_front = false;
			_power_out.turn_right_rear = false;
		}
		else if _turn_right_on.0 {
			_power_out.turn_right_front = _turn_right_on.1;
			_power_out.turn_right_rear = _turn_right_on.1;
			_power_out.turn_left_front = false;
			_power_out.turn_left_rear = false;
			
		}
		else {
			_power_out.turn_right_front = false;
			_power_out.turn_right_rear = false;	
			_power_out.turn_left_front = false;
			_power_out.turn_left_rear = false;
		}
	}
}

// Switches the lights according to driver input
//
// There are two modes:
//  * Ignition On:		When light is turned on, low beam and rear light will
//						be activated. This also allows the full beam to be turned on
//						when the driver input activated it.
//						When light is off, all head and read lights are turned off and
//						the full beam input will be ignored.
//  * Ignition Off:		When light is on, parking light is enabled, all other ligths stay off.
fn switch_light_signals(_input : &Input, _power_out : & mut PowerOutput) {

	if _input.ignition {
	
		match _input.light_on {
			true => {
				_power_out.head_light_lowbeam = true;
				_power_out.rear_light = true;
				_power_out.head_light_fullbeam = _input.full_beam;
			}
			false => {
				_power_out.head_light_lowbeam = false;
				_power_out.rear_light = false;
			}
		}
	} else {
		match _input.light_on {
			true => {
				_power_out.head_light_lowbeam = false;
				_power_out.head_light_parking = true;
				_power_out.rear_light = true;
			},
			false => {
				_power_out.head_light_lowbeam = false;
				_power_out.head_light_parking = false;
				_power_out.rear_light = false;
			}
		}
		_power_out.head_light_fullbeam = false;
	}	
}


// Switches the power output based on the input and current system state and
// returns the PowerOutput prefilled. The return value contains the state of the
// Power output as it should be applied by the hardware. 
fn switch_power_output(_system : &System, _input : &Input) -> PowerOutput {
	
	let mut power_output = PowerOutput {
		turn_left_front : false,
		turn_left_rear : false,
		turn_right_front : false,
		turn_right_rear : false,
		head_light_parking : false,
		head_light_lowbeam : false,
		head_light_fullbeam : false,
		rear_light : false,
		brake_light : _input.brake_front || _input.brake_rear,
		horn : _input.horn,
	};

	switch_turn_signals(&_system, &_input, & mut power_output);	
	switch_light_signals(&_input, & mut power_output);

	power_output
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