#[macro_use]
extern crate version_sync;

#[test]
fn test_html_root_url() {
    assert_html_root_url_updated!("src/lib.rs");
}
