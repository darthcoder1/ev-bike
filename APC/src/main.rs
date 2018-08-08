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

use rt::ExceptionFrame;

use hal::prelude::*;
use hal::stm32f103xx;
use hal::delay::Delay;
use embedded_hal::digital::OutputPin;

mod time;
mod logic;

entry!(main);

/*struct PowerChannelMapper
{
	enable_funcs : [fn(&OutputPin,bool);12];
}

fn enable_channel<T>(channel : T, _enabled : bool) 
	where T : OutputPin
{
	if _enabled {
		channel.set_high();
	} else {
		channel.set_low();
	}
}

impl PowerChannelMapper
{
	fn new() -> PowerChannelMapper{
		let mut _cortex_m = cortex_m::Peripherals::take().unwrap();
		let _stm32f103 = stm32f103xx::Peripherals::take().unwrap();

		let mut _flash = _stm32f103.FLASH.constrain();
		let mut _rcc = _stm32f103.RCC.constrain();

		let _clocks = _rcc.cfgr.freeze(& mut _flash.acr);
		_cortex_m.DWT.enable_cycle_counter();

		let mut gpioa = _stm32f103.GPIOA.split(& mut _rcc.apb2);
		let mut gpiob = _stm32f103.GPIOB.split(& mut _rcc.apb2);
		let mut gpioc = _stm32f103.GPIOC.split(& mut _rcc.apb2);

		let mut led = gpioc.pc13.into_push_pull_output(& mut gpioc.crh);
		
		let mut channel0_on = gpiob.pb5.into_push_pull_output(& mut gpiob.crl);
		let mut channel1_on = gpiob.pb6.into_push_pull_output(& mut gpiob.crl);

		let mapper = PowerChannelMapper {
			enable_funcs: [enable_channel]
		}
	}
	
	
}*/

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

	let mut led = gpioc.pc13.into_push_pull_output(& mut gpioc.crh);
	
	let mut channel0_on = gpiob.pb5.into_push_pull_output(& mut gpiob.crl);
	let mut channel1_on = gpiob.pb6.into_push_pull_output(& mut gpiob.crl);

	//let mut channel0_st = gpioa.pa5.into_floating_input(& mut gpioa.crl);
	//let mut channel1_st = gpioa.pa6.into_floating_input(& mut gpioa.crl);

	let mut delay = Delay::new(_cortex_m.SYST, _clocks);

	// setup the turn signal state
	let mut _system_state = logic::SystemState {
		turn_left : logic::State::Inactive,
		turn_right : logic::State::Inactive,
		hazard : logic::State::Inactive,
	};

    loop {
    	_system_state = logic::tick( _system_state, _clocks);
    }
 }

fn set_power_channel(_channel : logic::PowerChannel, _high : bool, _gpio_pin : impl OutputPin) {

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