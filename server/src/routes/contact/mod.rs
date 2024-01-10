use maud::{html, Markup};

// pub mod form_submission;

pub fn contact_page() -> Markup {
    html! {
        h1 { "Contact" }

        p { "O hai mark" }

        form
            class="flex flex-col gap-2 w-full max-w-2xl py-4"
            action="http://localhost:3000/contact"
            method="POST"
        {
            label {
                p { "Name" }
                input required name="name";
            }

            label {
                p { "Email" }
                input required name="email";
            }

            label {
                p { "Message" }
                textarea required name="message" {}
            }

            button { "Send" }
        }
    }
}
