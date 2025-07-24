use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use dbus_crossroads::IfaceBuilder;
use tracing::trace;
use crate::connection::DBusConnection;
use crate::UUID;

mod rdn;
mod homepage;
mod app_type;
mod description;

async fn get_data_directory(connection: Arc<DBusConnection>) -> PathBuf {
    trace!("Data directory empty attempting to call directories service");

    let proxy = connection.directories_service_apps_proxy();

    let as_string = UUID.to_string();

    let result: (String,) = proxy.method_call("voxels.directories", "data", (as_string,)).await.unwrap();

    PathBuf::from(result.0)
}

pub fn configure_interface(con: Arc<DBusConnection>, data_directory: Arc<RwLock<Option<PathBuf>>>, b: &mut IfaceBuilder<()>) {
    rdn::add_rdn_method_to_interface(con.clone(), data_directory.clone(), b);

    homepage::add_homepage_method_to_interface(con.clone(), data_directory.clone(), b);

    app_type::add_type_method_to_interface(con.clone(), data_directory.clone(), b);

    description::add_description_method_to_interface(con.clone(), data_directory.clone(), b);
}
