use common::models::task::Task;
use reqwasm::http::Request;
use yew::{function_component, html, use_effect_with_deps, use_state, Html, Properties};

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct TaskViewProps {
    pub task_global_id: String,
}

async fn get_task(id: &String) -> Task {
    let url = format!("/api/task/{}", id);

    Request::get(&url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

#[function_component(TaskView)]
pub fn task_view(props: &TaskViewProps) -> Html {
    let task_global_id = props.task_global_id.clone();
    let task = use_state(|| None);

    {
        let task = task.clone();
        use_effect_with_deps(
            move |_| {
                let task = task.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_task = get_task(&task_global_id).await;

                    task.set(Some(fetched_task));
                });

                // return for effect
                || ()
            },
            (),
        );
    }

    if let Some(t) = &*task {
        let res_file = match &t.res_file {
            Some(r) => r,
            None => "None",
        };

        html! {
            <>
                <table>
                <tr>
                <td>{"User ID"}</td>
                <td>{t.user_id.clone()}</td>
                </tr>

                                     <tr>
                        <td>{"Task Type"}</td>
                        <td>{t.task_type.clone()}</td>
                    </tr>
                    <tr>
                        <td>{"State"}</td>
                        <td>{t.state.clone()}</td>
                    </tr>
                    <tr>
                        <td>{"Source File"}</td>
                        <td>{t.src_file.clone()}</td>
                    </tr>
                    <tr>
                        <td>{"Result File"}</td>
                        <td>{res_file}</td>
                    </tr>
                </table>

                </>
        }
    } else {
        html! {<div>{"Loading ..."}</div>}
    }
}
