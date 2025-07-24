use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use dbus_crossroads::IfaceBuilder;
use tracing::{
    trace, info
};
use crate::connection::DBusConnection;
use crate::UUID;

use std::path::Path;
use libsql::{Builder, Connection};

mod rdn;
mod homepage;
mod app_type;
mod description;

async fn get_data_directory(connection: Arc<DBusConnection>) -> PathBuf {
    trace!("Data directory empty attempting to call directories service");

    let proxy = connection.directories_service_apps_proxy();

    let as_string = UUID.to_string();

    let result: (String,) = proxy.method_call("voxels.directories", "data", (as_string,)).await.unwrap();
    
    info!("Data directory acquired: {}", result.0);

    PathBuf::from(result.0)
}

async fn get_database(data_directory: &Path) -> Connection {
    let data_directory = data_directory.join("apps.sqlite");
    
    info!("getting database at: {}", data_directory.display());
    
    let db = Builder::new_local(data_directory.to_str().unwrap()).build().await.unwrap();
    
    db.connect().unwrap()
}

pub fn configure_interface(con: Arc<DBusConnection>, data_directory: Arc<RwLock<Option<PathBuf>>>, database: Arc<RwLock<Option<Connection>>>, b: &mut IfaceBuilder<()>) {
    rdn::add_rdn_method_to_interface(con.clone(), data_directory.clone(), database.clone(), b);

    homepage::add_homepage_method_to_interface(con.clone(), data_directory.clone(),database.clone(), b);

    app_type::add_type_method_to_interface(con.clone(), data_directory.clone(), database.clone(), b);

    description::add_description_method_to_interface(con.clone(), data_directory.clone(), database.clone(), b);
}
