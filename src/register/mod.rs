mod application;

use application::add_method_to_interface as add_application_method_to_interface;

use std::sync::Arc;
use dbus_crossroads::IfaceBuilder;
use crate::connection::DBusConnection;

pub fn configure_interface(con: Arc<DBusConnection>, b: &mut IfaceBuilder<()>) {
    add_application_method_to_interface(con.clone(), b)
}