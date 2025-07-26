mod connection;
mod get;
mod register;

use get::configure_interface as configure_get_interface;
use register::configure_interface as configure_register_interface;

use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use lib_voxels_application::application::application::{
    Application
};

use tracing::{
    Level,
    subscriber::set_global_default,
};

use tracing_subscriber::FmtSubscriber;

use uuid::Uuid;
use crate::connection::DBusConnection;

use libsql::Connection as DbConnection;

use lib_voxels_application::application::{
    DBUS_STANDARD_VOXELS_APPLICATIONS_GET_PATH,
};

lazy_static::lazy_static! {
    static ref APP_CONFIG: Application = toml::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/voxels.toml"))).expect("Unable to parse TOML from voxels.toml config file");

    static ref INTERFACE_NAME: &'static str = APP_CONFIG.rdn().name();

    static ref UUID: Uuid = APP_CONFIG.id();
}

fn setup_subscriber() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    set_global_default(subscriber)
        .expect("Failed to set subscriber");
}

#[tokio::main]
async fn main() {
    setup_subscriber();

    let dbus_connection_future = DBusConnection::connect(*INTERFACE_NAME, |err| {
        panic!("Lost connection to D-Bus: {}", err);
    });

    let dbus_connection = match dbus_connection_future.await {
        Ok(connection) => connection,
        Err(err) => panic!("Failed to connect to D-Bus: {}", err),
    };

    let data_directory: Arc<RwLock<Option<PathBuf>>> = Arc::new(RwLock::new(None));
    
    let database: Arc<RwLock<Option<DbConnection>>> = Arc::new(RwLock::new(None));

    let mut cr = dbus_crossroads::Crossroads::new();

    cr.set_async_support(Some((dbus_connection.raw(), Box::new(|x| { tokio::spawn(x); }))));

    let base_token = cr.register(*INTERFACE_NAME, |b| {
        configure_get_interface(dbus_connection.clone(), data_directory.clone(), database.clone(), b);
    });

    cr.insert(DBUS_STANDARD_VOXELS_APPLICATIONS_GET_PATH, &[base_token], ());

    let register_token = cr.register(*INTERFACE_NAME, |b| {
        configure_register_interface(dbus_connection.clone(), b);
    });

    cr.insert("/register", &[register_token], ());

    dbus_connection.start_receive(cr);

    loop {}
}
