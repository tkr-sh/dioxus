//! A tour of the rsx! macro
//! ------------------------
//!
//! This example serves as an informal quick reference of all the things that the rsx! macro can do.
//!
//! A full in-depth reference guide is available at: https://www.notion.so/rsx-macro-basics-ef6e367dec124f4784e736d91b0d0b19
//!
//! ### Elements
//! - Create any element from its tag
//! - Accept compile-safe attributes for each tag
//! - Display documentation for elements
//! - Arguments instead of String
//! - Text
//! - Inline Styles
//!
//! ## General Concepts
//! - Iterators
//! - Keys
//! - Match statements
//! - Conditional Rendering
//!
//! ### Events
//! - Handle events with the "onXYZ" syntax
//! - Closures can capture their environment with the 'a lifetime
//!
//!
//! ### Components
//! - Components can be made by specifying the name
//! - Components can be referenced by path
//! - Components may have optional parameters
//! - Components may have their properties specified by spread syntax
//! - Components may accept child nodes
//! - Components that accept "onXYZ" get those closures bump allocated
//!
//! ### Fragments
//! - Allow fragments using the built-in `Fragment` component
//! - Accept a list of vnodes as children for a Fragment component
//! - Allow keyed fragments in iterators
//! - Allow top-level fragments

fn main() {
    dioxus_desktop::launch(App);
}

use core::{fmt, str::FromStr};
use std::fmt::Display;

use baller::Baller;
use dioxus::prelude::*;

#[component]
fn App(cx: Scope) -> Element {
    let formatting = "formatting!";
    let formatting_tuple = ("a", "b");
    let lazy_fmt = format_args!("lazily formatted text");
    let asd = 123;
    cx.render(rsx! {
        div {
            // Elements
            div {}
            h1 {"Some text"}
            h1 {"Some text with {formatting}"}
            h1 {"Formatting basic expressions {formatting_tuple.0} and {formatting_tuple.1}"}
            h1 {"Formatting without interpolation " formatting_tuple.0 "and" formatting_tuple.1 }
            h2 {
                "Multiple"
                "Text"
                "Blocks"
                "Use comments as separators in html"
            }
            div {
                h1 {"multiple"}
                h2 {"nested"}
                h3 {"elements"}
            }
            div {
                class: "my special div",
                h1 {"Headers and attributes!"}
            }
            div {
                // pass simple rust expressions in
                class: lazy_fmt,
                id: format_args!("attributes can be passed lazily with std::fmt::Arguments"),
                class: "asd",
                class: "{asd}",
                // if statements can be used to conditionally render attributes
                class: if formatting.contains("form") { "{asd}" },
                div {
                    class: {
                        const WORD: &str = "expressions";
                        format_args!("Arguments can be passed in through curly braces for complex {WORD}")
                    }
                }
            }

            // Expressions can be used in element position too:
            rsx!(p { "More templating!" }),

            // Iterators
            (0..10).map(|i| rsx!(li { "{i}" })),

            // Iterators within expressions
            {
                let data = std::collections::HashMap::<&'static str, &'static str>::new();
                // Iterators *should* have keys when you can provide them.
                // Keys make your app run faster. Make sure your keys are stable, unique, and predictable.
                // Using an "ID" associated with your data is a good idea.
                data.into_iter().map(|(k, v)| rsx!(li { key: "{k}", "{v}" }))
            }

            // Matching
            match true {
                true => rsx!( h1 {"Top text"}),
                false => rsx!( h1 {"Bottom text"})
            }

            // Conditional rendering
            // Dioxus conditional rendering is based around None/Some. We have no special syntax for conditionals.
            // You can convert a bool condition to rsx! with .then and .or
            true.then(|| rsx!(div {})),

            // Alternatively, you can use the "if" syntax - but both branches must be resolve to Element
            if false {
                rsx!(h1 {"Top text"})
            } else {
                rsx!(h1 {"Bottom text"})
            }

            // Using optionals for diverging branches
            if true {
                Some(rsx!(h1 {"Top text"}))
            } else {
                None
            }

            // returning "None" without a diverging branch is a bit noisy... but rare in practice
            None as Option<()>,

            // can also just use empty fragments
            Fragment {}

            // Fragments let you insert groups of nodes without a parent.
            // This lets you make components that insert elements as siblings without a container.
            div {"A"}
            Fragment {
                div {"B"}
                div {"C"}
                Fragment {
                    "D"
                    Fragment {
                        "E"
                        "F"
                    }
                }
            }

            // Components
            // Can accept any paths
            // Notice how you still get syntax highlighting and IDE support :)
            Baller {}
            baller::Baller {}
            crate::baller::Baller {}

            // Can take properties
            Taller { a: "asd" }

            // Can take optional properties
            Taller { a: "asd" }

            // Can pass in props directly as an expression
            {
                let props = TallerProps {a: "hello", children: cx.render(rsx!(()))};
                rsx!(Taller { ..props })
            }

            // Spreading can also be overridden manually
            Taller {
                ..TallerProps { a: "ballin!", children: cx.render(rsx!(()) )},
                a: "not ballin!"
            }

            // Can take children too!
            Taller { a: "asd", div {"hello world!"} }

            // This component's props are defined *inline* with the `inline_props` macro
            WithInline { text: "using functionc all syntax" }

            // Components can be generic too
            // This component takes i32 type to give you typed input
            TypedInput::<i32> {}

            // Type inference can be used too
            TypedInput { initial: 10.0 }

            // geneircs with the `inline_props` macro
            Label { text: "hello geneirc world!" }
            Label { text: 99.9 }

            // Lowercase components work too, as long as they are access using a path
            baller::lowercase_component {}

            // For in-scope lowercase components, use the `self` keyword
            self::lowercase_helper {}

            // helper functions
            // Anything that implements IntoVnode can be dropped directly into Rsx
            helper(cx, "hello world!")

            // Strings can be supplied directly
            String::from("Hello world!")

            // So can format_args
            format_args!("Hello {}!", "world")

            // Or we can shell out to a helper function
            format_dollars(10, 50)
        }
    })
}

fn format_dollars(dollars: u32, cents: u32) -> String {
    format!("${dollars}.{cents:02}")
}

fn helper<'a>(cx: &'a ScopeState, text: &'a str) -> Element<'a> {
    cx.render(rsx! {
        p { "{text}" }
    })
}

// no_case_check disables PascalCase checking if you *really* want a snake_case component.
// This will likely be deprecated/removed in a future update that will introduce a more polished linting system,
// something like Clippy.
#[component(no_case_check)]
fn lowercase_helper(cx: Scope) -> Element {
    cx.render(rsx! {
        "asd"
    })
}

mod baller {
    use super::*;
    #[derive(Props, PartialEq, Eq)]
    pub struct BallerProps {}

    #[component]
    /// This component totally balls
    pub fn Baller(_cx: Scope<BallerProps>) -> Element {
        todo!()
    }

    // no_case_check disables PascalCase checking if you *really* want a snake_case component.
    // This will likely be deprecated/removed in a future update that will introduce a more polished linting system,
    // something like Clippy.
    #[component(no_case_check)]
    pub fn lowercase_component(cx: Scope) -> Element {
        cx.render(rsx! { "look ma, no uppercase" })
    }
}

#[derive(Props)]
pub struct TallerProps<'a> {
    /// Fields are documented and accessible in rsx!
    a: &'static str,
    children: Element<'a>,
}

/// Documention for this component is visible within the rsx macro
#[component]
pub fn Taller<'a>(cx: Scope<'a, TallerProps<'a>>) -> Element {
    cx.render(rsx! {
        &cx.props.children
    })
}

#[derive(Props, PartialEq, Eq)]
pub struct TypedInputProps<T> {
    #[props(optional, default)]
    initial: Option<T>,
}

#[allow(non_snake_case)]
pub fn TypedInput<T>(_: Scope<TypedInputProps<T>>) -> Element
where
    T: FromStr + fmt::Display,
    <T as FromStr>::Err: std::fmt::Display,
{
    todo!()
}

#[component]
fn WithInline<'a>(cx: Scope<'a>, text: &'a str) -> Element {
    cx.render(rsx! {
        p { "{text}" }
    })
}

#[component]
fn Label<T>(cx: Scope, text: T) -> Element
where
    T: Display,
{
    cx.render(rsx! {
        p { "{text}" }
    })
}
