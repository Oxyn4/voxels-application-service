mod connection;

use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use dbus::MethodErr;
use dbus::nonblock::MethodReply;
use dbus_crossroads::IfaceBuilder;
use lib_voxels_application::application::application::{
    Application
};

use tracing::{Level, info, trace};

use turso::{
    Builder as TursoBuilder,
};

use uuid::Uuid;
use crate::connection::DBusConnection;

lazy_static::lazy_static! {
    static ref APP_CONFIG: Application = toml::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/voxels.toml"))).expect("Unable to parse TOML from voxels.toml config file");

    static ref INTERFACE_NAME: &'static str = APP_CONFIG.rdn().name();

    static ref UUID: Uuid = APP_CONFIG.id();
}

fn get_rdn(uuid: Uuid) -> Result<(String,), MethodErr> {
    todo!()
}

fn get_homepage(uuid: Uuid) -> Result<(String,), MethodErr> {
    todo!()
}

fn get_type(uuid: Uuid) -> Result<(String,), MethodErr> {
    todo!()
}

fn get_description(uuid: Uuid) -> Result<(String,), MethodErr> {
    todo!()
}

fn configure_interface(dbus_connection: Arc<DBusConnection>, data_directory: Arc<RwLock<Option<PathBuf>>>, b: &mut IfaceBuilder<()>) {
    let data_directory_clone = data_directory.clone();

    let connection = dbus_connection.clone();

    b.method_with_cr_async("rdn", ("uuid",), ("rdn",), move |mut ctx, _, (uuid,): (String,)| {
        info!("method: 'rdn' called with uuid: {}", uuid);

        let data_directory_clone = data_directory_clone.clone();

        let connection = connection.clone();

        let parsed_uuid = Uuid::parse_str(uuid.as_str());

        async move {
            if parsed_uuid.is_err() {
                return ctx.reply(Err(MethodErr::failed("Invalid UUID")));
            }

            let uuid = parsed_uuid.unwrap();

            if uuid == *UUID {
                return ctx.reply(Ok((INTERFACE_NAME.to_owned(),)));
            }

            if data_directory_clone.read().unwrap().is_none() {
                trace!("Data directory empty attempting to call directories service");

                let proxy = connection.directories_service_apps_proxy();

                let as_string = UUID.to_string();

                let result: (String,) = proxy.method_call("voxels.directories", "data", (as_string,)).await.unwrap();

                let mut guard = data_directory_clone.write().unwrap();

                *guard = Some(PathBuf::from(&result.0));
            }

            let result = get_rdn(uuid).expect("failed to get rdn");

            ctx.reply(Ok(result))
        }
    });

    let data_directory_clone = data_directory.clone();

    let connection = dbus_connection.clone();

    b.method_with_cr_async("homepage", ("uuid",), ("homepage",), move |mut ctx, _, (uuid,): (String,)| {
        info!("method: 'homepage' called with uuid: {}", uuid);

        let data_directory_clone = data_directory_clone.clone();

        let connection = connection.clone();

        let parsed_uuid = Uuid::parse_str(uuid.as_str());

        async move {
            if parsed_uuid.is_err() {
                return ctx.reply(Err(MethodErr::failed("Invalid UUID")));
            }

            let uuid = parsed_uuid.unwrap();

            if uuid == *UUID {
                return ctx.reply(Ok((INTERFACE_NAME.to_owned(),)));
            }

            if data_directory_clone.read().unwrap().is_none() {
                trace!("Data directory empty attempting to call directories service");

                let proxy = connection.directories_service_apps_proxy();

                let as_string = UUID.to_string();

                let result: (String,) = proxy.method_call("voxels.directories", "data", (as_string,)).await.unwrap();

                let mut guard = data_directory_clone.write().unwrap();

                *guard = Some(PathBuf::from(&result.0));
            }

            let result = get_homepage(uuid).expect("failed to get homepage");

            ctx.reply(Ok(result))
        }
    });

    let data_directory_clone = data_directory.clone();

    let connection = dbus_connection.clone();

    b.method_with_cr_async("type", ("uuid",), ("type",), move |mut ctx, _, (uuid,): (String,)|{
        info!("method: 'type' called with uuid: {}", uuid);

        let data_directory_clone = data_directory_clone.clone();

        let connection = connection.clone();

        let parsed_uuid = Uuid::parse_str(uuid.as_str());

        async move {
            if parsed_uuid.is_err() {
                return ctx.reply(Err(MethodErr::failed("Invalid UUID")));
            }

            let uuid = parsed_uuid.unwrap();

            if uuid == *UUID {
                return ctx.reply(Ok((INTERFACE_NAME.to_owned(),)));
            }

            if data_directory_clone.read().unwrap().is_none() {
                trace!("Data directory empty attempting to call directories service");

                let proxy = connection.directories_service_apps_proxy();

                let as_string = UUID.to_string();

                let result: (String,) = proxy.method_call("voxels.directories", "data", (as_string,)).await.unwrap();

                let mut guard = data_directory_clone.write().unwrap();

                *guard = Some(PathBuf::from(&result.0));
            }

            let result = get_type(uuid).expect("failed to get type");

            ctx.reply(Ok(result))
        }
    });

    let data_directory_clone = data_directory.clone();

    let connection = dbus_connection.clone();

    b.method_with_cr_async("description", ("uuid",), ("description",), move |mut ctx, _, (uuid,): (String,)| {
        info!("method: 'description' called with uuid: {}", uuid);

        let data_directory_clone = data_directory_clone.clone();

        let connection = connection.clone();

        let parsed_uuid = Uuid::parse_str(uuid.as_str());

        async move {
            if parsed_uuid.is_err() {
                return ctx.reply(Err(MethodErr::failed("Invalid UUID")));
            }

            let uuid = parsed_uuid.unwrap();

            if uuid == *UUID {
                return ctx.reply(Ok((INTERFACE_NAME.to_owned(),)));
            }

            if data_directory_clone.read().unwrap().is_none() {
                trace!("Data directory empty attempting to call directories service");

                let proxy = connection.directories_service_apps_proxy();

                let as_string = UUID.to_string();

                let result: (String,) = proxy.method_call("voxels.directories", "data", (as_string,)).await.unwrap();

                let mut guard = data_directory_clone.write().unwrap();

                *guard = Some(PathBuf::from(&result.0));
            }

            let result = get_description(uuid).expect("failed to get description");

            ctx.reply(Ok(result))
        }
    });
}

fn setup_subscriber() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
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

    let directories_proxy = dbus_connection.directories_service_apps_proxy();

    let data_directory: Arc<RwLock<Option<PathBuf>>> = Arc::new(RwLock::new(None));

    let mut dbus_cr = dbus_crossroads::Crossroads::new();

    dbus_cr.set_async_support(Some((dbus_connection.raw(), Box::new(|x| { tokio::spawn(x); }))));

    let base_dbus_token = dbus_cr.register(*INTERFACE_NAME, |b| {
        configure_interface(dbus_connection.clone(), data_directory.clone(), b);
    });

    dbus_cr.insert("/get", &[base_dbus_token], ());

    dbus_connection.start_receive(dbus_cr);

    loop {}
}
