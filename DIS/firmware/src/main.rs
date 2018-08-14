
#![deny(unsafe_code)]

extern crate videocore;
extern crate egl;

use videocore::bcm_host;
use videocore::dispmanx;

fn main() {
    println!("Hello, world!");

    bcm_host::init();

    let display = dispmanx::display_open(0);
    let update_hndl = dispmanx::update_start(0);

    dispmanx::display_set_background(update_hndl, display, 255, 0, 0);
    dispmanx::update_submit_sync(update_hndl);
}
