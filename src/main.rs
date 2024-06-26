// use super::run;
use learn_wpgu::run;
use winit::error::EventLoopError;

extern crate pollster;

fn main() -> Result<(), EventLoopError>{
    pollster::block_on(run())
}
