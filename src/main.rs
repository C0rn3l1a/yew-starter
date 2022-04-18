mod components;

use crate::components::common::nav::NavBar;
use crate::components::video_details::VideoDetails;
use crate::components::video_list::{Video, VideosList};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let videos = vec![
        Video {
            id: 1,
            title: "Building and breaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];

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
                <VideosList videos={videos} on_click={on_video_select.clone()} />
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
