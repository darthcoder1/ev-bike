#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32f103xx_hal as hal;
extern crate embedded_hal;

mod time;
mod logic;

use rt::ExceptionFrame;

use hal::prelude::*;
use hal::stm32f103xx;
use hal::delay::Delay;
use logic::PowerChannel;

entry!(main);


// Main entry
fn main() -> ! {

	let mut _cortex_m = cortex_m::Peripherals::take().unwrap();
	let _stm32f103 = stm32f103xx::Peripherals::take().unwrap();

	let mut _flash = _stm32f103.FLASH.constrain();
	let mut _rcc = _stm32f103.RCC.constrain();

	let _clocks = _rcc.cfgr.freeze(& mut _flash.acr);
	_cortex_m.DWT.enable_cycle_counter();

	let mut gpioa = _stm32f103.GPIOA.split(& mut _rcc.apb2);
	let mut gpiob = _stm32f103.GPIOB.split(& mut _rcc.apb2);
	let mut gpioc = _stm32f103.GPIOC.split(& mut _rcc.apb2);

	let mut led = gpioc.pc13.into_push_pull_output(& mut gpioc.crh); led.set_low();
	
	
	// This is the mapping between the actual pins and the power channels they switch
	
	// acquire all the pins 
	let mut channel0 = gpiob.pb4.into_push_pull_output(& mut gpiob.crl);	channel0.set_low();
	let mut channel1 = gpiob.pb5.into_push_pull_output(& mut gpiob.crl);	channel1.set_low();
	let mut channel2 = gpiob.pb6.into_push_pull_output(& mut gpiob.crl);	channel2.set_low();
	let mut channel3 = gpiob.pb7.into_push_pull_output(& mut gpiob.crl);	channel3.set_low();
	let mut channel4 = gpiob.pb8.into_push_pull_output(& mut gpiob.crh);	channel4.set_low();
	let mut channel5 = gpiob.pb9.into_push_pull_output(& mut gpiob.crh);	channel5.set_low();
	let mut channel6 = gpiob.pb10.into_push_pull_output(& mut gpiob.crh);	channel6.set_low();
	let mut channel7 = gpiob.pb11.into_push_pull_output(& mut gpiob.crh);	channel7.set_low();
	let mut channel8 = gpiob.pb12.into_push_pull_output(& mut gpiob.crh);	channel8.set_low();
	let mut channel9 = gpiob.pb13.into_push_pull_output(& mut gpiob.crh);	channel9.set_low();
	//let mut channel10 = gpiob.pb14.into_push_pull_output(& mut gpiob.crh);
	//let mut channel11 = gpiob.pb15.into_push_pull_output(& mut gpiob.crh);	

	// create the mapping
	let mut power_channels = [ 
		PowerChannel::TurnLeftFront		(& mut channel0),
		PowerChannel::TurnLeftRear		(& mut channel1),
		PowerChannel::TurnRightFront	(& mut channel2),
		PowerChannel::TurnRightRear		(& mut channel3),
		PowerChannel::HeadLightParking	(& mut channel4),
		PowerChannel::HeadLightLowerBeam(& mut channel5),
		PowerChannel::HeadLightFullBeam	(& mut channel6),
		PowerChannel::RearLight			(& mut channel7),
		PowerChannel::BrakeLight		(& mut channel8),
		PowerChannel::Horn				(& mut channel9),
	];


	let mut delay = Delay::new(_cortex_m.SYST, _clocks);

	// setup the turn signal state
	let mut _system_state = logic::SystemState {
		turn_left : logic::State::Inactive,
		turn_right : logic::State::Inactive,
		hazard : logic::State::Inactive,
	};

    loop {
    	_system_state = logic::tick( _system_state, & mut power_channels, _clocks);
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