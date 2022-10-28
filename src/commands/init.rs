use std::{env, fs, io::ErrorKind};

pub fn run(_namespace: Option<String>) {
    if env::var("DOCKER_INIT").is_ok() {
        eprintln!("Init should not be run inside this docker image as it's already been run. Exiting...");
        return;
    }

    let config_dirname = dirs::data_dir()
        .expect("Data directory not found")
        .join("cloup");

    if let Err(e) = fs::create_dir(&config_dirname) {
        match e.kind() {
            ErrorKind::PermissionDenied => {
                eprintln!("Permission denied when creating config directory")
            }
            _ => (),
        }
    }

    let config_file = config_dirname.join(".config");
    if config_file.is_file() && !config_file.exists() {
        return;
    }

    // TODO: Improve config file, use config-managable crate for this

    fs::write(
        config_dirname.join(".config"),
        format!("template_dir={:?}", env::current_dir().unwrap()),
    )
    .expect("An error occurred when writing config file");

    println!("📚 Successfully made this the template directory for cloups");
}
