use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav() -> Html {
    html! {
        <nav>
            <ul>
                <li><a href="/" class="flex-left">{"Yew App"}</a></li>
                <li><a href="/about">{"About"}</a></li>
                <li><a href="/contact">{"Contact"}</a></li>
            </ul>
        </nav>
    }
}
