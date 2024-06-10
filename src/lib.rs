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


/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    let once = create_local_resource(|| (), |_| async move { load_data().await });
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors.get().into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

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
