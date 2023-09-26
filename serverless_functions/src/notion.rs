use crate::ContactFormSubmission;
use serde::Serialize;

static notion_api_token: &str = std::env!("NOTION_API_TOKEN");
static database_id: &str = std::env!("NOTION_DATABASE_ID");

pub async fn add_contact_form_submission_to_database(form_data: ContactFormSubmission) {
    let request_body = RequestBody::from_form_data(form_data);
    let serialized_request_body = serde_json::to_string(&request_body).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.notion.com/v1/pages")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", notion_api_token),
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header("Notion-Version", "2022-06-28")
        .body(serialized_request_body)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    dbg!(res);
}

#[derive(Serialize)]
struct RequestBody {
    parent: Parent,
    properties: Properties,
}

#[derive(Serialize)]
struct Parent {
    database_id: String,
}

#[derive(Serialize)]
struct Properties {
    name: TitleProperty,
    // email: EmailProperty,
    // message: TextProperty,
}

#[derive(Serialize)]
struct TitleProperty {
    title: Vec<Text>,
}

#[derive(Serialize)]
struct Text {
    content: String,
}

impl RequestBody {
    fn from_form_data(form_data: ContactFormSubmission) -> RequestBody {
        RequestBody {
            parent: Parent {
                database_id: database_id.to_string(),
            },
            properties: Properties {
                name: TitleProperty {
                    title: vec![Text {
                        content: form_data.name,
                    }],
                },
            },
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
