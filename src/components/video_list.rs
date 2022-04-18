use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>,
}

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    videos
        .iter()
        .map(|video| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_: MouseEvent| on_click.emit(video.clone()))
                // !notice there is no ";" in here, we are returning callback!!!
            };

            html! {
                <div class="card">
                    <p onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
                </div>
            }
        })
        .collect()
}
