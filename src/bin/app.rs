#![allow(non_snake_case)]

use anyhow::anyhow;
use dioxus::prelude::*;
use std::time::Duration;

/*
#[derive(PartialEq, Props, Clone)]
struct IpEntryProps {
    ip: String,
    date: String,
    source: IpSource,
}

impl Default for IpEntryProps {
    fn default() -> Self {
        Self {
            ip: "".to_string(),
            date: format!("{:?}", SystemTime::now()),
            source: IpSource::Network,
        }
    }
}

fn IpEntry(cx: Scope<IpEntryProps>) -> Element {
    cx.render(rsx! {
        span {
            "{cx.props.ip}"
        },
        span {
            "{cx.props.date}"
        }
    })
}
*/

/*
#[derive(Props)]
struct IpHistoryProps<'a> {
    pub ips: &'a HashMap<Ip, IpDetails>,
}

fn IpHistory<'a>(cx: Scope<'a, IpHistoryProps<'a>>) -> Element {
    cx.render(rsx! {
        ul {
            cx.props.ips.iter().map(|(ip, details)| rsx! {
                li {
                    "{ip} - {details.date}"
                }
            })
        }
    })
}
*/

fn main() {
    //let (sender, _receiver) = unbounded();

    // launch our IO thread
    std::thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            });
    });

    // launch our app on the current thread - important because we spawn a window
    //dioxus::desktop::launch_with_props(app, LocalIpProps { my_ip: None }, |c| c)
    dioxus_desktop::launch(app::app);
}
