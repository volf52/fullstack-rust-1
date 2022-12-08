use yew::{function_component, html, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct TaskViewProps {
    pub task_global_id: String,
}

#[function_component(TaskView)]
pub fn task_view(props: &TaskViewProps) -> Html {
    html! {}
}
