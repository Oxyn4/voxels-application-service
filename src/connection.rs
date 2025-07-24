use std::sync::Arc;
use std::time::Duration;
use dbus::channel::MatchingReceiver;
use dbus::message::MatchRule;
use dbus::nonblock::{Proxy, SyncConnection};
use dbus_tokio::connection::IOResourceError;

pub struct DBusConnection {
    connection: Arc<SyncConnection>,
    join_handle: tokio::task::JoinHandle<()>,
}

impl DBusConnection {
    pub async fn connect<F>(interface_name: &'static str, on_connection_loss: F) -> Result<Arc<Self>, dbus::Error>
    where
        F: FnOnce(IOResourceError) + Send + 'static,
    {
        let (resource, connection) = dbus_tokio::connection::new_session_sync()?;

        // if resource finished we lost connection
        let join_handle = tokio::spawn(async move {
            let err = resource.await;

            on_connection_loss(err);
        });

        connection.request_name(interface_name, false, true, false).await?;

        Ok(Arc::new(Self { connection, join_handle }))
    }

    pub fn start_receive(&self, mut crossroads: dbus_crossroads::Crossroads) {
        // receive events have crossroads handle dispatch
        self.connection.start_receive(MatchRule::new_method_call(), Box::new(move |message, connection| {
            crossroads.handle_message(message, connection).unwrap();

            true
        }));
    }

    pub fn raw(&self) -> Arc<SyncConnection> {
        self.connection.clone()
    }

    pub fn directories_service_apps_proxy(&self) -> Proxy<'_, Arc<SyncConnection>> {
        Proxy::new("voxels.directories", "/apps", Duration::from_millis(5000), self.connection.clone())
    }
}