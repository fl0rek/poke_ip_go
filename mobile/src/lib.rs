use mobile_entry_point::mobile_entry_point;

#[mobile_entry_point]
fn main() {
    dioxus_mobile::launch(poke_app::app)
}
