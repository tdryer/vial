use vial::util;

#[test]
fn content_type() {
    assert_eq!(util::content_type("file.png"), "image/png");
    assert_eq!(util::content_type("./path/to/file.png"), "image/png");
    assert_eq!(
        util::content_type("other/../path/to/some-file123.png"),
        "image/png"
    );
    assert_eq!(util::content_type("funny-meme.gif"), "image/gif");
    assert_eq!(util::content_type("/photo12401240.JPG"), "image/jpeg");
    assert_eq!(util::content_type("goo-times.jPeG"), "image/jpeg");
    assert_eq!(util::content_type("report.pdf"), "application/pdf");
    assert_eq!(
        util::content_type("/css/style.min.css"),
        "text/css; charset=utf8"
    );
    assert_eq!(
        util::content_type("some-v1.24.css"),
        "text/css; charset=utf8"
    );
    assert_eq!(util::content_type("index.htm"), "text/html; charset=utf8");
    assert_eq!(
        util::content_type("/users/profile.html"),
        "text/html; charset=utf8"
    );
    assert_eq!(util::content_type("robots.txt"), "text/plain; charset=utf8");
    assert_eq!(
        util::content_type("/short/story.text"),
        "text/plain; charset=utf8"
    );
    assert_eq!(util::content_type("Readme.md"), "text/plain; charset=utf8");
    assert_eq!(
        util::content_type("MANUAL.markdown"),
        "text/plain; charset=utf8"
    );
    assert_eq!(util::content_type("?"), "text/plain; charset=utf8");
}

#[test]
fn decode_form_value() {
    assert_eq!(
        util::decode_form_value("Well%2C+that%27s+just+great%21"),
        "Well, that's just great!"
    );
}

#[test]
fn http_current_date() {
    assert!(!util::http_current_date().is_empty());
}

#[test]
fn file_size() {
    assert_eq!(1052, util::file_size("LICENSE-MIT"));
    assert_eq!(25161, util::file_size("tests/assets/rfcs/rfc1288.txt"));
    assert_eq!(0, util::file_size("LICENSE-MADE-UP"));
}
