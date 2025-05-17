use build_const::ConstWriter;

fn main() {
    let licenses = match std::process::Command::new("cargo")
        .args(["license", "-j"])
        .output()
    {
        Ok(licenses) => licenses,
        Err(err) => {
            println!("cargo::error=Could not fetch licenses due to error: {err}");
            panic!("Could not fetch licenses due to error: {err}");
        }
    };

    let licenses = match String::from_utf8(licenses.stdout) {
        Ok(licenses) => licenses,
        Err(err) => {
            println!("cargo::error=Could not parse string due to error: {err}");
            panic!("Could not parse string due to error: {err}");
        }
    };

    let mut writer = match ConstWriter::for_build("licenses") {
        Ok(writer) => writer,
        Err(err) => {
            println!("cargo::error=Could not create ConstWriter due to error: {err}");
            panic!("Could not create ConstWriter due to error: {err}");
        }
    }
    .finish_dependencies();

    writer.add_value_raw(
        "LICENSES_STR",
        "&str",
        format!("r#\"{licenses}\"#").as_str(),
    );

    writer.finish();
}
