#[allow(non_snake_case)]

mod datapackage;
mod dashboard;

use dioxus::prelude::*;
use crate::datapackage::Datapackage;
use crate::dashboard::Dashboard;
use datapackage::*;


const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

pub static API_URL: &str = "https://backboard.vercel.app/datapackage.json";

// pub async fn get_data_package() -> Result<String, reqwest::Error> {
//     reqwest::get(API_URL).await?.json().await
// }


fn main() {
    // launch(App);
    launch(Dashboard);
}

fn App() -> Element {

    let mut data_package = use_resource(|| async move {
        reqwest::get(API_URL)
            .await
            .unwrap()
            .json::<Datapackage>()
            .await
    });

    // println!("{:?}", &*data_package.read_unchecked());

    rsx! {
        div { class: "min-h-screen bg-gradient-to-r from-accent to-base-100", "data-theme": "cupcake",
            div { class: "container prose pt-6  w-full",
                match &*data_package.read_unchecked() {
                    Some(Ok(response)) => {
                        let projects = response.resources.iter().find(|&x| x.name == "projects").unwrap();
                        let data = &projects.data.clone();
                        rsx! {
                            h1 { class: "text-primary-content text-", "{projects.name}" }
                            div { class: "pt-32 grid grid-cols-3 w-full grid-flow-row-dense mt-62",
                                for (i, project) in data.iter().enumerate() {
                                    Hexagon {
                                        offset: i > 0 && (i - 1) % 3 == 0,
                                        div { class: "flex flex-col content-center",
                                            p { class: "text-primary-content text-center",
                                                "{project.name}"
                                                // "{project.progress.unwrap()}"
                                            }
                                            progress {
                                                class: "progress progress secondary w-32",
                                                value: "{project.score.unwrap()}", max: "100"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(Err(e)) => rsx! { div { "Loading data package failed: {e}" } },
                    None => rsx! { div { "Loading data package..." } },
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct HexagonProps {
    children: Element,
    #[props(default)]
    offset: bool
}
fn Hexagon(props: HexagonProps) -> Element {
    if props.offset {
        rsx! {
            div { class: "mt-[-62] mb-10",
                div {
                    class: "cursor-pointer hex mask mask-hexagon-2 glass w-64 h-64 hover:bg-primary flex items-center justify-center",
                    {props.children}
                }
            }
        }
    } else {
        rsx! {
            div { class: "mt-16 mb-[-62]",
                div { class: "cursor-pointer hex mask mask-hexagon-2 glass w-64 h-64 hover:bg-primary flex items-center justify-center",
                    {props.children}
                }
            }
        }
    }
}