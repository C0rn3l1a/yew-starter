mod components;

use crate::components::common::nav::NavBar;
use crate::components::video_details::VideoDetails;
use crate::components::video_list::{Video, VideosList};
use reqwasm::http::Request;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> =
                        // Request::get("https://yew.rs/tutorial/data.json")
                        Request::get("/tutorial/data.json")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    html! {
        <>
            <NavBar></NavBar>
            <main class="landing">
                <header>
                    <h1>{ "List of Videos" }</h1>
                </header>
                <section>
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
                </section>
                <section>
                    { for details }
                </section>
            </main>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
