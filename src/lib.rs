mod app;
mod ip;
mod ip_api;
mod poke_api;
mod pokedex;
mod pokemon;
mod storage_api;

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn start_app() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Trace)
            .with_tag("template"),
    );

    dioxus_desktop::wry::android_binding!(com_example, template, _start_app, dioxus_desktop::wry);
}

#[cfg(target_os = "android")]
fn _start_app() {
    if let Err(err) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(main)) {
        eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
        std::process::abort();
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn main() {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    std::env::set_var("RUST_BACKTRACE", "1");

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    pretty_env_logger::init();

    log::info!("hwgm");
    dioxus_desktop::launch(app::app);
}

#[cfg(target_family = "wasm")]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    dioxus_web::launch(app::app);
}
