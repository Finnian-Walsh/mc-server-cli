use server::{
    config,
    executables::{self, Executable},
    home,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executable = executables::get(Executable::Serverup, "release");
    let dst = home::get()?.join(config::get("path")?).join("serverup");
    executables::sudo_update(executable, dst)?;
    Ok(())
}
