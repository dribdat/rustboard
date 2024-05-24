#![allow(non_snake_case)]

use std::future;
use std::io::Result;
use std::time::Duration;
use dioxus::html::{td, time};
use dioxus::prelude::*;
use gloo_timers::callback::Timeout;
use crate::API_URL;
use crate::datapackage::*;
use futures_util::stream::StreamExt;
use gloo_timers::future::IntervalStream;
use wasm_bindgen_futures::spawn_local;

pub fn Dashboard() -> Element {

    let mut show_alert = use_signal(|| false);
    let mut clock_running = use_signal(|| false);


    rsx! {
        div { "data-theme": "light", class: "flex h-screen bg-gradient-to-r from-blue-400 to-purple-300",
            div { class: "w-1/5", Teams {}}
            div { class: "flex flex-col flex-1 0",
                div {
                    class: "flex-1 relative",
                    div { class: "w-full h-full flex items-center justify-center",
                        span { class: "text-4xl", "Open Farming Hackdays 2024"}
                    }
                    if *show_alert.read() == true {
                        {rsx! {
                            div {
                                class: "absolute w-full top-0",
                                Alert {}
                            }
                        }}
                    }
                }
                div { class: "h-1/6", Chart {}}
                div { class: "h-1/6 w-full", Clock {clock_running}}
            }
            div { class: "w-1/5", Posts {}}
        }
        div { class: "fixed bottom-10 left-10 flex gap-2",
            button {class: "btn btn-primary", onclick: move |_| show_alert.toggle(), "Show Notification"}
            button {class: "btn btn-primary", onclick: move |_| clock_running.toggle(), "Start Clock"}
        }
    }
}

pub fn Alert() -> Element {
    rsx! {
        div {
            role: "alert",
            class: "flex alert w-[95%] m-auto mt-5 bg-gradient-to-r from-yellow-400 to-yellow-300 shadow",
            img {
                class: "w-48 h-48",
                src: "https://upload.wikimedia.org/wikipedia/commons/2/2f/Rickrolling_QR_code.png"
            }
            div { class: "card bg-base-100 flex-1 h-full",
                div { class: "card-body h-full flex items-center justify-center",
                    svg {
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        class: "stroke-info shrink-0 w-12 h-12 animate-bounce",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                        }
                    }
                    span { class: "text-2xl text-center",
                        "Food orders are now open."
                        br{}
                        "Please order your lunch until 10:30"
                    }
                }
            }
        }
    }
}

pub fn Teams() -> Element {
    pub static PROJECTS_URL: &str = "https://hack.farming.opendata.ch/api/event/current/projects.json";
    let mut projects= use_resource(|| async move {
        reqwest::get(PROJECTS_URL)
            .await
            .unwrap()
            .json::<ProjectsRoot>()
            .await
    });

    rsx! {
        match &*projects.read_unchecked() {
            Some(Ok(response)) => {
                let mut projects = response.projects.clone();
                projects.sort_by_key(|a| a.score);
                projects = projects.into_iter().rev().collect();
                rsx! {
                    div {class: "h-full flex flex-col gap-1 p-2 content-stretch place-content-stretch",
                        for project in projects {
                            div{ class: "flex-grow card glass p-1",
                                div {"{project.name}" }
                                progress {class: "progress w-full", value: "{project.score}", max: 100}
                            }
                        }
                    }
                }
            }
            Some(Err(e)) => {rsx!{"Loading Error: {e:?}" }}
            None => {rsx!{LoadingIcon{}}}
        }
    }
}

pub fn Posts() -> Element {
    pub static POSTS_URL: &str = "https://hack.farming.opendata.ch/api/project/posts.json?limit=12";
    let mut posts = use_resource(|| async move {
        reqwest::get(POSTS_URL)
            .await
            .unwrap()
            .json::<ActivityRoot>()
            .await
    });

    rsx! {
        match &*posts.read_unchecked() {
            Some(Ok(response)) => {
                let posts = &response.activities;
                rsx! {
                    div {class: "h-full flex flex-col gap-1 p-2 content-stretch place-content-stretch",
                        for post in posts {
                            div{ class: "flex-grow card glass p-1 flex items-center justify-center",  "{post.action}"}
                        }
                    }
                }
            }
            Some(Err(e)) => {rsx!{"Loading Error: {e:?}"}}
            None => {rsx!{LoadingIcon{}}}
        }
    }
}

#[component]
pub fn Clock(clock_running: Signal<bool>) -> Element {
    let full_duration = 86400;
    let mut time_left_seconds = use_signal(|| full_duration);
    let mut tdhms = use_signal(|| (full_duration, 0,0,0,0));

    use_future(move || async move {
        IntervalStream::new(1_000).for_each(move |_| {
            if clock_running() {
                let t = tdhms().0 -1;
                let days = t / (60 * 60 * 24);
                let hours = t / (60 * 60) % 24;
                let minutes = t / 60 % 60;
                let seconds = t % 60;
                tdhms.set((t, days, hours, minutes, seconds));
            }
            future::ready(())
        }).await;
    });

    // use_future(move || async move {
    //     wasm_bindgen_futures::spawn_local()
    // }); /*    use_future(move || async move {
        // let mut time_left_seconds = time_left_seconds.to_owned();
    // });

    // let countdown_task = use_coroutine(|rx: UnboundedReceiver<()>| {
    //     async move {
    //         loop {
    //         }
    //     }
    // });

    let (t, days, hours, minutes, seconds) = tdhms();

    rsx! {
        div {
            class: "grid w-full h-full grid-flow-col gap-5 text-center cols-4 place-items-stretch p-5",
            div {
                class: "flex flex-col p-2 bg-neutral rounded-box text-neutral-content items-center justify-center",
                span { class: "countdown font-mono text-5xl",
                    span {
                        style: "--value:{days}",
                    }
                }
                "days"
            }
            div {
                class: "flex flex-col p-2 bg-neutral rounded-box text-neutral-content items-center justify-center",
                span { class: "countdown font-mono text-5xl",
                    span {
                        style: "--value:{hours}",
                    }
                }
                "hours"
            }
            div {
                class: "flex flex-col p-2 bg-neutral rounded-box text-neutral-content items-center justify-center",
                span { class: "countdown font-mono text-5xl",
                    span {
                        style: "--value:{minutes}",
                    }
                }
                "minutes"
            }
            div {
                class: "flex flex-col p-2 bg-neutral rounded-box text-neutral-content items-center justify-center",
                span { class: "countdown font-mono text-5xl",
                    span {
                        style: "--value:{seconds}",
                    }
                }
                "seconds"
            }
        }
    }
}

pub fn Chart() -> Element {
    rsx!(
        svg {
            class: "w-full h-full p-5",
            "viewBox": "0 0 1000 110",
            polyline {
                fill: "none",
                stroke: "#0074d9",
                "stroke-width": "2",
                points: "\
                   00,120
                   120,60
                   240,80
                   260,20
                   300,80
                   300,80
                   320,60
                   440,100
                   460,90
                   480,80
                   500, 110
                   620, 10
                   640, 70
                   660, 100
                   700, 100
                   780, 40
                   820, 10
                   840, 100
                   860, 100
                   880, 120
                   900, 60
                   920, 70
                   940, 80
                  1000, 23
                "
            }
        }
    )
}

pub fn LoadingIcon() -> Element {
    rsx! {
        div { class: "flex w-full h-full items-center justify-center",
            span { class: "loading loading-spinner loading-lg" }
        }
    }
}