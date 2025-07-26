use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use dbus::MethodErr;
use dbus_crossroads::{Context, IfaceBuilder};
use tracing::info;
use libsql::{Connection, Database};
use uuid::Uuid;
use crate::connection::DBusConnection;
use crate::{INTERFACE_NAME, UUID};

use super::{
    get_data_directory,
    get_database,
};

use lib_voxels_application::application::application::{
    DBUS_STANDARD_VOXELS_APPLICATIONS_RDN_METHOD,
};


fn get_rdn(uuid: Uuid) -> Result<(String,), MethodErr> {
    todo!()
}

pub async fn handle_method(con: Arc<DBusConnection>, mut ctx: Context, parsed_uuid: Result<Uuid, uuid::Error>, data_directory: Arc<RwLock<Option<PathBuf>>>, database: Arc<RwLock<Option<Connection>>>) -> PhantomData<(String,)> {
    if parsed_uuid.is_err() {
        return ctx.reply(Err(MethodErr::failed("Invalid UUID")));
    }

    let uuid = parsed_uuid.unwrap();

    if uuid == *UUID {
        return ctx.reply(Ok((INTERFACE_NAME.to_owned(),)));
    }

    // if we haven't obtained the data directory from application service do so now
    if data_directory.read().expect("").is_none() {
        *data_directory.write().unwrap() = Some(get_data_directory(con.clone()).await);
        
        let data_path  = data_directory.read().expect("").clone().unwrap();
        
        // now we need to get the database which depends on data path
        *database.write().unwrap() = Some(get_database(&data_path.as_path()).await);
    }

    let result = get_rdn(uuid).expect("failed to get rdn");

    ctx.reply(Ok(result))
}

pub fn add_method_to_interface(con: Arc<DBusConnection>, data_directory: Arc<RwLock<Option<PathBuf>>>, database: Arc<RwLock<Option<Connection>>>, b: &mut IfaceBuilder<()>) {
    b.method_with_cr_async(DBUS_STANDARD_VOXELS_APPLICATIONS_RDN_METHOD, ("uuid",), ("rdn",), move |ctx, _, (uuid,): (String,)| {
        info!("method: 'rdn' called with uuid: {}", uuid);

        let parsed_uuid = Uuid::parse_str(uuid.as_str());

        handle_method(con.clone(), ctx, parsed_uuid, data_directory.clone(), database.clone())
    });
}