use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
	html! {
			<main>
					<img class="logo" src="public/logo.png" alt="Yew logo" />
					<h1>{ "Hey!" }</h1>
					<span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
			</main>
	}
}
