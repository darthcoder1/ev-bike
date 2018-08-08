pub use time::TimeStamp;

use time;

// This indicates s state and if active, since which tick
pub enum State
{
	Active(TimeStamp),
	Inactive,
}

// This is the system state that needs to be kept around for timing relevant
// state
pub struct SystemState
{
	pub turn_left : State,
	pub turn_right : State,
	pub hazard : State,
}

// These are the inputs coming from the driver controlls
pub struct Input
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
pub enum PowerChannel
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

	NumChannels = 10,
}

//fn initialize_power_channel(_channel : PowerChannel, )

// This describes how the power ouptput needs to be swithed, which
// outputs to open and which to close
pub struct PowerOutput
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
		hazard_light : true,
		light_on : false,
		full_beam : false,
		horn : false, 
		kill_switch : false,
		side_stand : false,
	};

	input
}


fn update_state(_prev_state : State, _input_flag : bool) -> State {
	
	if _input_flag  {
		match _prev_state {
			State::Active(_time) => _prev_state,
			State::Inactive => State::Active(time::device_get_ticks()),
		}
	}
	else {
		State::Inactive
	}
}

fn update_system_state(_system : SystemState, _input : &Input) -> SystemState {

	SystemState {
        turn_left : update_state(_system.turn_left, _input.turn_left),
	    turn_right : update_state(_system.turn_right, _input.turn_right),
	    hazard : update_state(_system.hazard, _input.hazard_light)
    }
}

fn caclulate_turn_signal(_state : &State, _cur_time : TimeStamp, _on_time : u32, _off_time : u32) -> bool {

	match _state {
		State::Active(start_time) => {
			let _time_passed = _cur_time.0.wrapping_sub(start_time.0);
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
fn switch_turn_signals(_system_state : &SystemState, _input : &Input, _clocks : &time::Clocks, _power_out : & mut PowerOutput)
{
	let current_time = time::device_get_ticks();
	let _one_sec_in_ticks = time::time_ms_to_ticks(&_clocks, 1000);

	let _hazard_on = match _system_state.hazard {
		State::Active(_start_time) => (true, caclulate_turn_signal(&_system_state.hazard, current_time, _one_sec_in_ticks, _one_sec_in_ticks)),
		State::Inactive => (false, false),
	};
	
	let _turn_left_on = match _system_state.turn_left {
		State::Active(_start_time) => (true, caclulate_turn_signal(&_system_state.turn_left, current_time, _one_sec_in_ticks, _one_sec_in_ticks)),
		State::Inactive => (false, false),
	};
	
	let _turn_right_on = match _system_state.turn_right {
		State::Active(_start_time) => (true, caclulate_turn_signal(&_system_state.turn_right, current_time, _one_sec_in_ticks, _one_sec_in_ticks)),
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
fn switch_power_output(_system : &SystemState, _input : &Input, _clock : &time::Clocks) -> PowerOutput {
	
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

	switch_turn_signals(&_system, &_input, &_clock, & mut power_output);	
	switch_light_signals(&_input, & mut power_output);

	power_output
}

fn apply_power_output(_power_out : PowerOutput) {
	/*let _stm32f103 = stm32f103xx::Peripherals::take().unwrap();
	let mut _rcc = _stm32f103.RCC.constrain();

	let mut gpioc = _stm32f103.GPIOC.split(& mut _rcc.apb2);
	let mut gpiob = _stm32f103.GPIOB.split(& mut _rcc.apb2);

	let mut led = gpioc.pc13.into_push_pull_output(& mut gpioc.crh);
	let mut channel0 = gpiob.pb5.into_push_pull_output(& mut gpiob.crl);

	//let mut channel : i32 = channel0;*/
}

pub fn tick(_system_state : SystemState, _clocks : time::Clocks) -> SystemState
{
    let _input = read_input();

    let _new_system_state = update_system_state(_system_state, &_input);
    let _power_out = switch_power_output(&_new_system_state, &_input, &_clocks);
    
    if _power_out.turn_left_front {
//        channel0_on.set_high();
//        led.set_low();
    } else {
//        channel0_on.set_low();
//        led.set_high();	
    }

    apply_power_output(_power_out);
    _new_system_state
    
    // TEST
    //delay.delay_ms(500_u16);
    //led.set_high();
    //delay.delay_ms(500_u16);
    //led.set_low();

    // 3999753

    
    // apply power state to hardware

    // read diagnosis from PFETs

    // output telemetry data
}
