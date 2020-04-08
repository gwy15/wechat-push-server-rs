use crate::errors::Result;
use crate::models;
use diesel::RunQueryDsl;

use diesel::prelude::PgConnection;

pub fn insert_message(msg: models::Message, con: &PgConnection) -> Result<()> {
    use crate::schema::messages::dsl::*;
    diesel::insert_into(messages).values(&msg).execute(con)?;
    Ok(())
}

fn find_message_by_uuid(uuid: String) -> Result<Option<models::Message>> {
    // TODO:
    Ok(None)
}
