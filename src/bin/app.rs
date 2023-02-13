#[cfg(not(target_family = "wasm"))]
fn main() {
    pretty_env_logger::init();
    //let (sender, _receiver) = unbounded();

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
