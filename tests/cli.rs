use minreq;
use snapbox::cmd::{cargo_bin, Command};

#[test]
fn without_args_check_reachability() -> Result<(), Box<dyn std::error::Error>> {
    let image_url = String::from_utf8(yesno().assert().success().get_output().stdout.clone())?;
    let status_code = minreq::head(image_url.trim())
        .send()
        .map(|resp| resp.status_code)?;
    assert_eq!(status_code, 200);
    Ok(())
}

#[test]
fn with_yes_arg() {
    yesno()
        .arg("yes")
        .assert()
        .stdout_matches(url("/yes/[..].gif"));
}

#[test]
fn with_no_arg() {
    yesno()
        .arg("no")
        .assert()
        .stdout_matches(url("/no/[..].gif"));
}

#[test]
fn with_maybe_arg() {
    yesno()
        .arg("maybe")
        .assert()
        .stdout_matches(url("/maybe/[..].gif"));
}

#[test]
fn with_unknown_arg() {
    yesno()
        .arg("unknown")
        .assert()
        .stderr_matches_path("tests/snapshots/cli_error.txt");
}

#[test]
fn with_help_arg() {
    yesno()
        .arg("--help")
        .assert()
        .stdout_matches_path("tests/snapshots/cli_help.txt");
}

fn yesno() -> Command {
    Command::new(cargo_bin!("yesno"))
}

fn url(path: &str) -> String {
    let mut url = String::from("https://yesno.wtf/assets");
    url.push_str(path);
    url.push_str("\n");
    url
}
