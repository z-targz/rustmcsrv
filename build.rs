use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
    process::Command,
};

fn main() {
    if !registry_exists() {
        download_server_jar_from_mojang();
        generate_data();
    }
}

fn download_server_jar_from_mojang() {
    let path = Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).join("server.jar");

    if std::fs::exists(path.clone()).unwrap_or(false) {
        return;
    }

    let resp = reqwest::blocking::get(
        "https://piston-data.mojang.com/v1/objects/450698d1863ab5180c25d7c804ef0fe6369dd1ba/server.jar"
    ).expect("request failed");

    let bytes = resp.bytes().unwrap().into_iter().collect::<Vec<u8>>();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .unwrap();
    file.write_all(bytes.as_slice())
        .expect("File system access isn't working.");
}

fn registry_exists() -> bool {
    let path = Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).join("generated");

    if std::fs::exists(path.join("data")).unwrap_or(false)
        && std::fs::exists(path.join("reports/registries.json")).unwrap_or(false)
    {
        return true;
    }
    false
}

fn generate_data() {
    let output = Command::new("java")
        .args([
            "-DbundlerMainClass=net.minecraft.data.Main",
            "-jar",
            "server.jar",
            "--reports",
            "--server",
        ])
        .output()
        .unwrap();
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
