use dioxus::prelude::*;
use dioxus_web::Config;
use web_sys::window;

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div { h1 { "thing 1" } }
        div { h2 { "thing 2" } }
        div {
            h2 { "thing 2" }
            "asd"
            "asd"
            Bapp {}
        }
        (0..10).map(|f| rsx!{
            div {
                "thing {f}"
            }
        })
    })
}

#[allow(non_snake_case)]
fn Bapp(cx: Scope) -> Element {
    cx.render(rsx! {
        div { h1 { "thing 1" } }
        div { h2 { "thing 2" } }
        div {
            h2 { "thing 2" }
            "asd"
            "asd"
        }
    })
}

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let mut dom = VirtualDom::new(app);
    let _ = dom.rebuild();

    let pre = dioxus_ssr::pre_render(&dom);
    tracing::trace!("{}", pre);

    // set the inner content of main to the pre-rendered content
    window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("main")
        .unwrap()
        .set_inner_html(&pre);

    // now rehydrate
    dioxus_web::launch_with_props(app, (), Config::new().hydrate(true));
}
