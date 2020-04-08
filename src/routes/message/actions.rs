use crate::errors::Result;
use crate::models;
use uuid::Uuid;

use diesel::prelude::*;

pub fn insert_message(msg: models::Message, con: &PgConnection) -> Result<()> {
    use crate::schema::messages::dsl::*;
    diesel::insert_into(messages).values(&msg).execute(con)?;
    Ok(())
}

pub fn find_message_by_uuid(uuid: Uuid, con: &PgConnection) -> Result<Option<models::Message>> {
    use crate::schema::messages::dsl::*;
    let mut msgs = messages
        .filter(id.eq(uuid))
        .limit(1)
        .load::<models::Message>(&*con)?;
    Ok(msgs.pop())
}
