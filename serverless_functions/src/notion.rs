use crate::ContactFormSubmission;
use notion::models::PageCreateRequest;

pub async fn add_contact_form_submission_to_database(form_data: ContactFormSubmission) {
    let properties = hashmap! {
        name: form_data.name,
        email: form_data.email,
        message: form_data.message,
    };

    let page = PageCreateRequest {
        parent: get_database_id(),
        properties,
    };

    notion_api.create_page(page).await.unwrap();
}

fn get_api() {
    let api_token = std::env!("NOTION_API_TOKEN");
    dbg!(&api_token);

    NotionApi::new(api_token)
}

fn get_database_id() {
    let database_id = std::env!("NOTION_DATABASE_ID");
    dbg!(&database_id);

    Parent::DatabaseId(DatabaseId::new(database_id))
}
