use dioxus::prelude::*;

#[inline_props]
pub fn Contact(cx: Scope) -> Element {
    render! {
        h1 { "Contact" }

        p { "O hai mark" }

        form {
            class: "flex flex-col gap-2 w-full max-w-2xl py-4",
            action: "http://localhost:8787/contact",
            method: "POST",
            label {
                p { "Name" }
                input { required: true, name: "name" }
            }

            label {
                p { "Email" }
                input { required: true, name: "email" }
            }

            label {
                p { "Message" }
                textarea { required: true, name: "message" }
            }

            button { "Send" }
        }
    }
}
