
use lib_voxels_directories as vxapp;

fn data() -> Result<(String,), dbus_crossroads::MethodErr> {
    use vxapp::voxels::voxels_xdg::data::DataDirectoryResolver;

    let fs = vxapp::filesystem::DefaultFsInt::default();
    let env = vxapp::environment_variables::DefaultEnvInt::default();

    let base_verifier = vxapp::voxels::voxels_xdg::xdg::data::DefaultDataVerifier::new(fs);

    let base = vxapp::voxels::voxels_xdg::xdg::data::DataDirectory::new(env, base_verifier);

    let resolver = vxapp::voxels::voxels_xdg::data::DataDirectory::new(base);

    let data_path = resolver.resolve();

    Ok((data_path.unwrap().into_os_string().into_string().unwrap(),))
}

fn config() -> Result<(String,), dbus_crossroads::MethodErr> {
    use vxapp::voxels::voxels_xdg::config::ConfigDirectoryResolver;

    let fs = vxapp::filesystem::DefaultFsInt::default();
    let env = vxapp::environment_variables::DefaultEnvInt::default();

    let base_verifier = vxapp::voxels::voxels_xdg::xdg::config::DefaultConfigVerifier::new(fs);

    let base = vxapp::voxels::voxels_xdg::xdg::config::ConfigDirectory::new(env, base_verifier);

    let resolver = vxapp::voxels::voxels_xdg::config::ConfigDirectory::new(base);

    let config_path = resolver.resolve();

    Ok((config_path.unwrap().into_os_string().into_string().unwrap(),))
}

fn runtime() -> Result<(String,), dbus_crossroads::MethodErr> {
    use vxapp::voxels::voxels_xdg::runtime::RuntimeDirectoryResolver;

    let fs = vxapp::filesystem::DefaultFsInt::default();
    let env = vxapp::environment_variables::DefaultEnvInt::default();

    let base_verifier = vxapp::voxels::voxels_xdg::xdg::runtime::DefaultRuntimeVerifier::new(fs);

    let base = vxapp::voxels::voxels_xdg::xdg::runtime::RuntimeDirectory::new(env, base_verifier);

    let resolver = vxapp::voxels::voxels_xdg::runtime::RuntimeDirectory::new(base);

    let runtime_path = resolver.resolve();

    Ok((runtime_path.unwrap().into_os_string().into_string().unwrap(),))
}
fn state() -> Result<(String,), dbus_crossroads::MethodErr> {
    use vxapp::voxels::voxels_xdg::state::StateDirectoryResolver;

    let fs = vxapp::filesystem::DefaultFsInt::default();
    let env = vxapp::environment_variables::DefaultEnvInt::default();

    let base_verifier = vxapp::voxels::voxels_xdg::xdg::state::DefaultStateVerifier::new(fs);

    let base = vxapp::voxels::voxels_xdg::xdg::state::StateDirectory::new(env, base_verifier);

    let resolver = vxapp::voxels::voxels_xdg::state::StateDirectory::new(base);

    let state_path = resolver.resolve();

    Ok((state_path.unwrap().into_os_string().into_string().unwrap(),))
}

#[lib_voxels_directories::lib_voxels_application::proc::main]
fn main() {
    const INTERFACE_NAME: &str = "org.voxels.directories";

    let dbus_connection = dbus::blocking::Connection::new_session().expect("Failed to connect to session DBus");

    dbus_connection.request_name(INTERFACE_NAME, false, true, false).expect("Failed to secure name");

    let mut dbus_crossroads = dbus_crossroads::Crossroads::new();


    let dbus_token = dbus_crossroads.register(INTERFACE_NAME, |b| {
        b.method("data", (), ("data_path",), |_context: &mut dbus_crossroads::Context, _, (): ()| -> Result<(String,), dbus_crossroads::MethodErr> {
            let result: Result<(String,), dbus_crossroads::MethodErr> = data();

            return result;
        });
        b.method("state", (), ("state_path",), |_context: &mut dbus_crossroads::Context, _, (): ()| -> Result<(String,), dbus_crossroads::MethodErr> {
            let result: Result<(String,), dbus_crossroads::MethodErr> = state();

            return result;
        });
        b.method("runtime", (), ("runtime_path",), |_context: &mut dbus_crossroads::Context, _, (): ()| -> Result<(String,), dbus_crossroads::MethodErr> {
            let result: Result<(String,), dbus_crossroads::MethodErr> = runtime();

            return result;
        });
        b.method("config", (), ("config_path",), |_context: &mut dbus_crossroads::Context, _, (): ()| -> Result<(String,), dbus_crossroads::MethodErr> {
            let result: Result<(String,), dbus_crossroads::MethodErr> = config();

            return result;
        });
    });

    dbus_crossroads.insert("/org/voxels/directories", &[dbus_token], ());

    dbus_crossroads.serve(&dbus_connection).expect("could not serve dbus service");
}
