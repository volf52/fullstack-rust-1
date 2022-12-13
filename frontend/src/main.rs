use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

use frontend::components::task_view::TaskView;

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[at("/task/:task_global_id")]
    TaskView { task_global_id: String },

    #[at("/test")]
    Test,
}

fn switch(route: Route) -> Html {
    log::info!("In routes: {:?}", route);

    match route {
        Route::TaskView { task_global_id } => html! {
            <TaskView task_global_id={task_global_id} />
        },
        Route::Test => html! { <div>{"Hello"}</div>},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("hello there");

    yew::Renderer::<App>::new().render();
}
