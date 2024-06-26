mod app;

use winit::{error::EventLoopError, window::Window};

use crate::app::App;


pub async fn run() -> Result<(), EventLoopError>{
    env_logger::init();


    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window_attributes = Window::default_attributes().with_title("A fantastic window!");
    let window = event_loop.create_window(window_attributes).unwrap();

    let mut app: App<()> = App::new(&window).await;
    // run()
    event_loop.run_app(&mut app)

}


