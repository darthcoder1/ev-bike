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
	let mut channel01 = gpioa.pa6.into_push_pull_output(& mut gpioa.crl);	channel01.set_low();
	let mut channel02 = gpioa.pa7.into_push_pull_output(& mut gpioa.crl);	channel02.set_low();
	let mut channel03 = gpiob.pb0.into_push_pull_output(& mut gpiob.crl);	channel03.set_low();
	let mut channel04 = gpiob.pb1.into_push_pull_output(& mut gpiob.crl);	channel04.set_low();
	let mut channel05 = gpiob.pb10.into_push_pull_output(& mut gpiob.crh);	channel05.set_low();
	let mut channel06 = gpiob.pb11.into_push_pull_output(& mut gpiob.crh);	channel06.set_low();

	let mut channel07 = gpioa.pa5.into_push_pull_output(& mut gpioa.crl);	channel07.set_low();
	let mut channel08 = gpioa.pa4.into_push_pull_output(& mut gpioa.crl);	channel08.set_low();
	let mut channel09 = gpioa.pa3.into_push_pull_output(& mut gpioa.crl);	channel09.set_low();
	let mut channel10 = gpioa.pa2.into_push_pull_output(& mut gpioa.crl);	channel10.set_low();
	let mut channel11 = gpioa.pa1.into_push_pull_output(& mut gpioa.crl); 	channel11.set_low();
	let mut channel12 = gpioa.pa0.into_push_pull_output(& mut gpioa.crl);  	channel11.set_low();	
	
	// create the mapping
	let mut power_channels = [ 
		PowerChannel::TurnLeftFront		(& mut channel01),
		PowerChannel::TurnLeftRear		(& mut channel02),
		PowerChannel::TurnRightFront	(& mut channel03),
		PowerChannel::TurnRightRear		(& mut channel04),
		PowerChannel::HeadLightParking	(& mut channel05),
		PowerChannel::HeadLightLowerBeam(& mut channel06),
		PowerChannel::HeadLightFullBeam	(& mut channel07),
		PowerChannel::RearLight			(& mut channel08),
		PowerChannel::BrakeLight		(& mut channel09),
		PowerChannel::Horn				(& mut channel10),
		PowerChannel::Unused0			(& mut channel11),
		PowerChannel::Unused1			(& mut channel12),
	];


	// setup the turn signal state
	let mut _system_state = logic::SystemState {
		turn_left : logic::State::Inactive,
		turn_right : logic::State::Inactive,
		hazard : logic::State::Inactive,
	};

    loop {
    	
		_system_state = logic::tick( _system_state, & mut power_channels, _clocks);

		// read diagnosis from PFETs

    	// output telemetry data
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