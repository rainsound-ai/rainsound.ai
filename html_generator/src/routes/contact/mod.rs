use dioxus::prelude::*;

#[inline_props]
pub fn Contact(cx: Scope) -> Element {
    render! {
        h1 { "Contact" }
        p { "This is the contact page." }

        form { action: "http://localhost:8787/contact", method: "POST",
            label { "Name" }
            input { required: true, name: "name" }

            label { "Email" }
            input { required: true, name: "email" }

            label { "Message" }
            textarea { required: true, name: "message" }

            button { "Send" }
        }
    }
}
