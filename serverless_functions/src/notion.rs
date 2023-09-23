// use crate::ContactFormSubmission;
// use notion::models::PageCreateRequest;

// pub async fn add_to_database(form_data: ContactFormSubmission) {
//     let properties = hashmap! {
//         name: form_data.name,
//         email: form_data.email,
//         message: form_data.message,
//     };

//     let page = PageCreateRequest {
//         parent: get_database_id(),
//         properties,
//     };

//     notion_api.create_page(page).await.unwrap();
// }

// fn get_api() {
//     let api_token = std::env::var("NOTION_API_TOKEN");

//     NotionApi::new(api_token)
// }

// fn get_database_id() {
//     let database_id = std::env::var("NOTION_DATABASE_ID");

//     Parent::DatabaseId(DatabaseId::new(database_id))
// }
