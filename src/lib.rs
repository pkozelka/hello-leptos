use leptos::*;
use leptos::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Items {
    items: Vec<DataItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DataItem {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub price: f64,
}

async fn load_data() -> Result<Vec<DataItem>> {
    // let request = reqwasm::http::Request::get("http://127.0.0.1:3000/data.json");
    let request = reqwasm::http::Request::get("/data.json");
    let data = request.send().await?.json::<Items>().await?;
    Ok(data.items)
}

#[component]
pub fn App() -> impl IntoView {
    let once = create_local_resource(|| (), |_| async move { load_data().await });
    let fallback = move |errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect_view()
            })
        };

        view! {
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };
    view! {
        <ErrorBoundary fallback>
            <div class="container">
                <table>
                <tr>
                    <th>Name</th>
                    <th>Description</th>
                    <th>Price</th>
                </tr>
                    {
                        move || once.get()
                            .unwrap_or(Ok(Vec::new()))
                            .map(|items| {
                                items.into_iter().map(|item| {
                                    view! {
                                    <tr>
                                        <td>{item.name}</td>
                                        <td>{item.description}</td>
                                        <td>{format!("Price: ${:.2}", item.price)}</td>
                                    </tr>
                                    }
                                }).collect_view()
                            })
                    }
                </table>
            </div>
        </ErrorBoundary>
    }
}
