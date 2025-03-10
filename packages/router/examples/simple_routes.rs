use dioxus::prelude::*;
use dioxus_router::prelude::*;
use std::str::FromStr;

#[cfg(feature = "liveview")]
#[tokio::main]
async fn main() {
    use axum::{extract::ws::WebSocketUpgrade, response::Html, routing::get, Router};

    let listen_address: std::net::SocketAddr = ([127, 0, 0, 1], 3030).into();
    let view = dioxus_liveview::LiveViewPool::new();
    let app = Router::new()
        .fallback(get(move || async move {
            Html(format!(
                r#"
                    <!DOCTYPE html>
                    <html>
                        <head></head>
                        <body><div id="main"></div></body>
                        {glue}
                    </html>
                "#,
                glue = dioxus_liveview::interpreter_glue(&format!("ws://{listen_address}/ws"))
            ))
        }))
        .route(
            "/ws",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    _ = view
                        .launch(dioxus_liveview::axum_socket(socket), Root)
                        .await;
                })
            }),
        );

    println!("Listening on http://{listen_address}");

    axum::Server::bind(&listen_address.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "liveview"))]
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dioxus_desktop::launch(Root);

    #[cfg(target_arch = "wasm32")]
    dioxus_web::launch(root);
}

#[cfg(feature = "liveview")]
#[component]
fn Root(cx: Scope) -> Element {
    render! { Router::<Route> {} }
}

#[cfg(not(feature = "liveview"))]
#[component]
fn Root(cx: Scope) -> Element {
    render! { Router::<Route> {} }
}

#[component]
fn UserFrame(cx: Scope, user_id: usize) -> Element {
    render! {
        pre { "UserFrame{{\n\tuser_id:{user_id}\n}}" }
        div { background_color: "rgba(0,0,0,50%)",
            "children:"
            Outlet::<Route> {}
        }
    }
}

#[component]
fn Route1(cx: Scope, user_id: usize, dynamic: usize, query: String, extra: String) -> Element {
    render! {
        pre {
            "Route1{{\n\tuser_id:{user_id},\n\tdynamic:{dynamic},\n\tquery:{query},\n\textra:{extra}\n}}"
        }
        Link {
            to: Route::Route1 {
                user_id: *user_id,
                dynamic: *dynamic,
                query: String::new(),
                extra: extra.clone() + ".",
            },
            "Route1 with extra+\".\""
        }
        p { "Footer" }
        Link {
            to: Route::Route3 {
                dynamic: String::new(),
            },
            "Home"
        }
    }
}

#[component]
fn Route2(cx: Scope, user_id: usize) -> Element {
    render! {
        pre { "Route2{{\n\tuser_id:{user_id}\n}}" }
        (0..*user_id).map(|i| rsx!{ p { "{i}" } }),
        p { "Footer" }
        Link {
            to: Route::Route3 {
                dynamic: String::new(),
            },
            "Home"
        }
    }
}

#[component]
fn Route3(cx: Scope, dynamic: String) -> Element {
    let current_route = use_route(cx)?;
    let current_route_str = use_ref(cx, String::new);
    let parsed = Route::from_str(&current_route_str.read());

    let site_map = Route::SITE_MAP
        .iter()
        .flat_map(|seg| seg.flatten().into_iter())
        .collect::<Vec<_>>();

    let navigator = use_navigator(cx);

    render! {
        input {
            oninput: move |evt| {
                *current_route_str.write() = evt.value();
            },
            value: "{current_route_str.read()}"
        }
        "dynamic: {dynamic}"
        Link { to: Route::Route2 { user_id: 8888 }, "hello world link" }
        button {
            disabled: !navigator.can_go_back(),
            onclick: move |_| {
                navigator.go_back();
            },
            "go back"
        }
        button {
            disabled: !navigator.can_go_forward(),
            onclick: move |_| {
                navigator.go_forward();
            },
            "go forward"
        }
        button {
            onclick: move |_| {
                navigator.push("https://www.google.com");
            },
            "google link"
        }
        p { "Site Map" }
        pre { "{site_map:#?}" }
        p { "Dynamic link" }
        match parsed {
            Ok(route) => {
                if route != current_route {
                    render! {
                        Link {
                            to: route.clone(),
                            "{route}"
                        }
                    }
                }
                else {
                    None
                }
            }
            Err(err) => {
                render! {
                    pre {
                        color: "red",
                        "Invalid route:\n{err}"
                    }
                }
            }
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[nest("/test")]
        // Nests with parameters have types taken from child routes
        #[nest("/user/:user_id")]
            // Everything inside the nest has the added parameter `user_id: usize`
            // UserFrame is a layout component that will receive the `user_id: usize` parameter
            #[layout(UserFrame)]
                #[route("/:dynamic?:query")]
                Route1 {
                    // The type is taken from the first instance of the dynamic parameter
                    user_id: usize,
                    dynamic: usize,
                    query: String,
                    extra: String,
                },
                #[route("/hello_world")]
                // You can opt out of the layout by using the `!` prefix
                #[layout(!UserFrame)]
                Route2 { user_id: usize },
            #[end_layout]
        #[end_nest]
    #[end_nest]
    #[redirect("/:id/user", |id: usize| Route::Route3 { dynamic: id.to_string()})]
    #[route("/:dynamic")]
    Route3 { dynamic: String },
}
