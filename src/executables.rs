use std::path;

pub enum Executable {
    Server,
    Serverup,
    ServerupUpdater,
}

pub fn get(executable: Executable, build_mode: &str) -> String {
    format!(
        "target{}{}{}{}",
        path::MAIN_SEPARATOR_STR,
        build_mode,
        path::MAIN_SEPARATOR_STR,
        match executable {
            Executable::Server => "server",
            Executable::Serverup => "serverup",
            Executable::ServerupUpdater => "serverup_updater",
        }
    )
}
