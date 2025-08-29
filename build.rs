use std::{
    env,
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
};

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=");

    let cargo_manifest_dir = &env::var("CARGO_MANIFEST_DIR").unwrap();
    let contact_path = Path::new(&cargo_manifest_dir).join("contact");

    assert!(
        !contact_path.is_dir(),
        "contact is a directory, but it should be a file"
    );

    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(contact_path)
    {
        writeln!(file, "none")?;
    }

    Ok(())
}
