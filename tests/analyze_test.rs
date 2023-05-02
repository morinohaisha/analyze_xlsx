use analyze_xlsx::analyze::analyze;

#[test]
fn test_analyze() {
    let res = analyze("tests/in.xlsx").unwrap();
    assert_eq!("[\"#<氏名>\",\"#<年齢>\",\"#<画像>\"]", res);
}