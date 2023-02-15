#[cfg(target_os = "android")]
mod android {

    pub fn init_logging() {
        android_logger::init_once(
            android_logger::Config::default()
                .with_min_level(log::Level::Trace)
                .with_tag("poke-ip-go"),
        );
    }

    pub fn run_app() {
        dioxus_mobile::launch(app::app)
        /*
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("A fantastic window!")
            .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
            .build(&event_loop)
            .unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            println!("{:?}", event);

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => (),
            }
        });
        */
    }
}

#[cfg(all(not(target_os = "android"), not(target_family = "wasm")))]
mod desktop {
    pub fn init_logging() {
        pretty_env_logger::init();
    }
    pub fn run_app() {
        dioxus_desktop::launch(app::app);
    }
}

#[cfg(target_family = "wasm")]
mod wasm {
    pub fn init_logging() {
        console_log::init_with_level(log::Level::Debug).expect("Failed to setup logging");
    }

    pub fn run_app() {
        dioxus_web::launch(app::app)
    }
}

#[cfg(target_os = "android")]
use android::*;
#[cfg(all(not(target_os = "android"), not(target_family = "wasm")))]
use desktop::*;
#[cfg(target_family = "wasm")]
use wasm::*;

/*
#[cfg(not(target_family = "wasm"))]
fn main() {
    // launch our IO thread
    std::thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            });
    });

    // launch our app on the current thread - important because we spawn a window
    //dioxus::desktop::launch_with_props(app, LocalIpProps { my_ip: None }, |c| c)
    dioxus_desktop::launch(app::app);
}

#[cfg(target_family = "wasm")]
pub fn main() {
    console_log::init_with_level(log::Level::Debug).expect("Failed to setup logging");
    dioxus_web::launch(app::app)
}
*/

//#[cfg_attr(target_os = "android", mobile_entry_point::mobile_entry_point)]
#[cfg(not(target_os = "android"))]
fn main() {
    init_logging();
    run_app();
}

#[cfg(target_os = "android")]
fn main() {}
/*
#[cfg(target_os = "android")]
fn main() {
    app::start_app();
}
*/
