use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use dbus_crossroads::IfaceBuilder;
use crate::connection::DBusConnection;

mod rdn;
mod homepage;
mod app_type;
mod description;

pub fn configure_interface(con: Arc<DBusConnection>, data_directory: Arc<RwLock<Option<PathBuf>>>, b: &mut IfaceBuilder<()>) {
    rdn::add_rdn_method_to_interface(con.clone(), data_directory.clone(), b);

    homepage::add_homepage_method_to_interface(con.clone(), data_directory.clone(), b);

    app_type::add_type_method_to_interface(con.clone(), data_directory.clone(), b);

    description::add_description_method_to_interface(con.clone(), data_directory.clone(), b);
}
