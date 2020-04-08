use crate::errors::Error;
use crate::models;

async fn post_new_message(title: String, body: String) {
    // TODO:
}

fn find_message_by_uuid(uuid: String) -> Result<Option<models::Message>, Error> {
    // TODO:
    Ok(None)
}
