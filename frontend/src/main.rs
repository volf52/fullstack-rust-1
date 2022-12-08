use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

use frontend::components::task_view::TaskView;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[at("/task/:task_global_id")]
    TaskView { task_global_id: String },
}

fn switch(route: Route) -> Html {
    match route {
        Route::TaskView { task_global_id } => html! {
            <TaskView task_global_id={task_global_id} />
        },
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

    yew::Renderer::<App>::new().render();
}
