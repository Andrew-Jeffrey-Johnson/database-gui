#[path = "../src/app/my_text/mod.rs"] mod my_text;

#[test]
fn test_label_capture() {
    let text: String = "Information Technology Group".to_string();
    let jargon = my_text::get_jargon();
    let acronyms = my_text::get_acronyms();
    let pkg = my_text::label_capture(text, &jargon, &acronyms);
    let ctrlPkg: my_text::LabelPkg = my_text::LabelPkg {
        text: "Information Technology Group".to_string(),
        tooltip: Some("ITG".to_string()),
        category: my_text::TextCategory::Jargon,
    };
    assert_eq!(pkg, ctrlPkg);
}
