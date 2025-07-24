use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use dbus::MethodErr;
use dbus_crossroads::{Context, IfaceBuilder};
use tracing::info;
use uuid::Uuid;
use crate::connection::DBusConnection;
use crate::{INTERFACE_NAME, UUID};

use super::get_data_directory;

fn get_description(uuid: Uuid) -> Result<(String,), MethodErr> {
    todo!()
}

pub async fn handle_description_method_call(con: Arc<DBusConnection>, mut ctx: Context, parsed_uuid: Result<Uuid, uuid::Error>, data_directory: Arc<RwLock<Option<PathBuf>>>) -> PhantomData<(String,)> {
    if parsed_uuid.is_err() {
        return ctx.reply(Err(MethodErr::failed("Invalid UUID")));
    }

    let uuid = parsed_uuid.unwrap();

    if uuid == *UUID {
        return ctx.reply(Ok((INTERFACE_NAME.to_owned(),)));
    }

    if data_directory.read().expect("").is_none() {
        *data_directory.write().expect("") = Some(get_data_directory(con.clone()).await);
    }

    let result = get_description(uuid).expect("failed to get description");

    ctx.reply(Ok(result))
}

pub fn add_description_method_to_interface(con: Arc<DBusConnection>, data_directory: Arc<RwLock<Option<PathBuf>>>, b: &mut IfaceBuilder<()>) {
    b.method_with_cr_async("description", ("uuid",), ("description",), move |ctx, _, (uuid,): (String,)| {
        info!("method: 'description' called with uuid: {}", uuid);

        let parsed_uuid = Uuid::parse_str(uuid.as_str());

        handle_description_method_call(con.clone(), ctx, parsed_uuid, data_directory.clone())
    });
}