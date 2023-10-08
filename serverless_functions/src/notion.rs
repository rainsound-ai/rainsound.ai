use crate::routes::contact::form_submission::ContactFormSubmission;
use serde::Serialize;
use worker::{Fetch, Headers, Method, Request, RequestInit};

static notion_api_token: &str = std::env!("NOTION_API_TOKEN");
static database_id: &str = std::env!("NOTION_DATABASE_ID");

pub async fn add_contact_form_submission_to_database(form_data: ContactFormSubmission) {
    let request_body = RequestBody::from_form_data(form_data);
    let serialized_request_body = serde_json::to_string(&request_body).unwrap();

    let mut headers = Headers::new();

    let bearer = format!("Bearer {}", notion_api_token);
    headers.set("Authorization", &bearer).unwrap();
    headers.set("Content-Type", "application/json").unwrap();
    headers.set("Notion-Version", "2022-06-28").unwrap();

    let mut request_init = RequestInit::new();
    request_init
        .with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(serialized_request_body.into()));

    let url = "https://api.notion.com/v1/pages";

    let request = Request::new_with_init(url, &request_init).unwrap();
    let response = Fetch::Request(request)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    // console_debug!("Response from Notion API: {}", response);
}

#[derive(Serialize)]
struct RequestBody {
    parent: Parent,
    properties: Properties,
    children: Vec<ParagraphBlock>,
}

#[derive(Serialize)]
struct Parent {
    database_id: String,
}

#[derive(Serialize)]
struct Properties {
    name: TitleProperty,
    email: EmailProperty,
}

#[derive(Serialize)]
struct TitleProperty {
    title: Vec<TitleElement>,
}

#[derive(Serialize)]
struct TitleElement {
    text: Text,
}

#[derive(Serialize)]
struct Text {
    content: String,
}

#[derive(Serialize)]
struct EmailProperty {
    email: String,
}

#[derive(Serialize)]
struct RichTextElement {
    r#type: &'static str,
    text: Text,
}

impl RichTextElement {
    fn new(text: String) -> Self {
        RichTextElement {
            r#type: "text",
            text: Text { content: text },
        }
    }
}

#[derive(Serialize)]
struct ParagraphBlock {
    object: &'static str, // Always "block"
    r#type: &'static str, // Always "paragraph"
    paragraph: Paragraph,
}

impl ParagraphBlock {
    fn new(text: String) -> Self {
        ParagraphBlock {
            object: "block",
            r#type: "paragraph",
            paragraph: Paragraph {
                rich_text: vec![RichTextElement::new(text)],
            },
        }
    }
}

#[derive(Serialize)]
struct Paragraph {
    rich_text: Vec<RichTextElement>,
}

impl RequestBody {
    fn from_form_data(form_data: ContactFormSubmission) -> RequestBody {
        RequestBody {
            parent: Parent {
                database_id: database_id.to_string(),
            },
            properties: Properties {
                name: TitleProperty {
                    title: vec![TitleElement {
                        text: Text {
                            content: form_data.name,
                        },
                    }],
                },
                email: EmailProperty {
                    email: form_data.email,
                },
                // message: vec![RichTextElement {
                //     r#type: "text",
                //     plain_text: form_data.message,
                // }],
            },
            children: vec![ParagraphBlock::new(form_data.message)],
        }
    }
}

/*
curl 'https://api.notion.com/v1/pages' \
  -H 'Authorization: Bearer '"$NOTION_API_KEY"'' \
  -H "Content-Type: application/json" \
  -H "Notion-Version: 2022-06-28" \
  --data '{
    "parent": { "database_id": "d9824bdc84454327be8b5b47500af6ce" },
  "icon": {
      "emoji": "🥬"
  },
    "cover": {
        "external": {
            "url": "https://upload.wikimedia.org/wikipedia/commons/6/62/Tuscankale.jpg"
        }
    },
    "properties": {
        "Name": {
            "title": [
                {
                    "text": {
                        "content": "Tuscan Kale"
                    }
                }
            ]
        },
        "Description": {
            "rich_text": [
                {
                    "text": {
                        "content": "A dark green leafy vegetable"
                    }
                }
            ]
        },
        "Food group": {
            "select": {
                "name": "Vegetable"
            }
        },
        "Price": { "number": 2.5 }
    },
    "children": [
        {
            "object": "block",
            "type": "heading_2",
            "heading_2": {
                "rich_text": [{ "type": "text", "text": { "content": "Lacinato kale" } }]
            }
        },
        {
            "object": "block",
            "type": "paragraph",
            "paragraph": {
                "rich_text": [
                    {
                        "type": "text",
                        "text": {
                            "content": "Lacinato kale is a variety of kale with a long tradition in Italian cuisine, especially that of Tuscany. It is also known as Tuscan kale, Italian kale, dinosaur kale, kale, flat back kale, palm tree kale, or black Tuscan palm.",
                            "link": { "url": "https://en.wikipedia.org/wiki/Lacinato_kale" }
                        }
                    }
                ]
            }
        }
    ]
}'
 */
