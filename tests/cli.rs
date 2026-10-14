use assert_cmd::Command;
use predicates::prelude::*;

fn term_img() -> Command {
    Command::cargo_bin("term-img").expect("binary should build")
}

fn sample_png(dir: &std::path::Path) -> std::path::PathBuf {
    let path = dir.join("sample.png");
    image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
        8,
        4,
        image::Rgba([200, 40, 40, 255]),
    ))
    .save(&path)
    .unwrap();
    path
}

#[test]
fn help_is_printed() {
    term_img()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Display images in your terminal"));
}

#[test]
fn version_matches_crate() {
    term_img()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn missing_source_exits_with_code_2() {
    term_img()
        .assert()
        .code(2)
        .stderr(predicate::str::contains("No image source provided"));
}

#[test]
fn zero_width_is_rejected() {
    term_img()
        .args(["--width", "0", "sample.png"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("greater than 0"));
}

#[test]
fn zero_height_is_rejected() {
    term_img()
        .args(["--height", "0", "sample.png"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("greater than 0"));
}

#[test]
fn missing_file_reports_error() {
    term_img()
        .arg("/definitely/not/here_12345.png")
        .assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn renders_a_local_png() {
    let dir = tempfile::tempdir().unwrap();
    let path = sample_png(dir.path());

    term_img()
        .args(["--width", "4", "--protocol", "halfblocks"])
        .arg(&path)
        .assert()
        .success();
}

#[test]
fn renders_from_stdin() {
    let dir = tempfile::tempdir().unwrap();
    let bytes = std::fs::read(sample_png(dir.path())).unwrap();

    term_img()
        .args(["--width", "4", "--protocol", "halfblocks", "-"])
        .write_stdin(bytes)
        .assert()
        .success();
}

#[test]
fn verbose_status_goes_to_stderr() {
    let dir = tempfile::tempdir().unwrap();
    let path = sample_png(dir.path());

    term_img()
        .args(["--verbose", "--width", "4", "--protocol", "halfblocks"])
        .arg(&path)
        .assert()
        .success()
        .stderr(predicate::str::contains("Displaying image"));
}
