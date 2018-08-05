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

fn device_get_ticks() -> TimeStamp {
	// TODO: get time from device
	TimeStamp(0)
}

fn time_milliseconds_to_ticks(ms : i32) -> i32 {
	ms
}

enum State
{
	Active(TimeStamp),
	Inactive,
}

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
		State::Active(startTime) => {
			let _time_passed = _cur_time.0 - startTime.0;
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

fn switch_power_output(_system : &System, _input : &Input) -> PowerOutput {
	
	let current_time = device_get_ticks();
	let _one_sec_in_ticks = time_milliseconds_to_ticks(1000);

	let _hazard_on = match _system.hazard {
		State::Active(start_time) => (true, caclulate_turn_signal(&_system.hazard, current_time, _one_sec_in_ticks, _one_sec_in_ticks)),
		State::Inactive => (false, false),
	};
	
	let _turn_left_on = match _system.turn_left {
		State::Active(start_time) => (true, caclulate_turn_signal(&_system.turn_left, current_time, _one_sec_in_ticks, _one_sec_in_ticks)),
		State::Inactive => (false, false),
	};
	
	let _turn_right_on = match _system.turn_right {
		State::Active(start_time) => (true, caclulate_turn_signal(&_system.turn_right, current_time, _one_sec_in_ticks, _one_sec_in_ticks)),
		State::Inactive => (false, false),
	};

	let mut power_output = PowerOutput {
		turn_left_front : false,
		turn_left_rear : false,
		turn_right_front : false,
		turn_right_rear : false,
		head_light_parking : false,
		head_light_lowbeam : false,
		head_light_fullbeam : false,
		rear_light : false,
		brake_light : false,
		horn : false,
	};

	if _hazard_on.0 {
		power_output.turn_right_front = _hazard_on.1;
		power_output.turn_right_rear = _hazard_on.1;
		power_output.turn_left_front = _hazard_on.1;
		power_output.turn_left_rear = _hazard_on.1;
	}
	else {
		if _turn_left_on.0 {
			power_output.turn_left_front = _turn_left_on.1;
			power_output.turn_left_rear = _turn_left_on.1;
			power_output.turn_right_front = false;
			power_output.turn_right_rear = false;
		}
		else if _turn_right_on.0 {
			power_output.turn_right_front = _turn_right_on.1;
			power_output.turn_right_rear = _turn_right_on.1;
			power_output.turn_left_front = false;
			power_output.turn_left_rear = false;
			
		}
		else {
			power_output.turn_right_front = false;
			power_output.turn_right_rear = false;	
			power_output.turn_left_front = false;
			power_output.turn_left_rear = false;
		}
	}
	

	power_output
}

fn main() -> ! {

	let mut _system_state = System {
		turn_left : State::Inactive,
		turn_right : State::Inactive,
		hazard : State::Inactive,
	};

    loop {
    	let _input = read_input();

		_system_state = update_system_state(_system_state, &_input);

    	let _power_out = switch_power_output(&_system_state, &_input);
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