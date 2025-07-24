use std::marker::PhantomData;
use std::sync::Arc;
use dbus_crossroads::IfaceBuilder;
use crate::connection::DBusConnection;

async fn handle_method() -> PhantomData<()> {
    todo!()
}

pub fn add_method_to_interface(con: Arc<DBusConnection>, b: &mut IfaceBuilder<()>) {
    b.method_with_cr_async("app", (), (), |_ctx, _, ()| {
        handle_method()
    });
}