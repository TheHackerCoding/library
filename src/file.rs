pub fn default_location() -> String {
    let result: String;
    let username = whoami::username();
    match whoami::platform() {
        whoami::Platform::Windows => result = format!("C:/{}/AppData/Local/_null/lib.null", username),
        whoami::Platform::Linux => result = format!("/home/{}/.config/_null/lib.null", username),
        _ => panic!("Unknown platform")
    }
    result

}