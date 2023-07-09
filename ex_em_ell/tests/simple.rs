#[derive(Debug, PartialEq, ex_em_ell::FromXmlDocument, ex_em_ell::ToXmlDocument)]
struct Example {
    field: String,
}

#[test]
fn test_from_xml() {
    let input = "<example><field>value</field></example>";
    let expected = Example {
        field: "value".to_string(),
    };
}

#[test]
fn test_to_xml() {
    let input = Example {
        field: "value".to_string(),
    };
    let expected = "<example><field>value</field></example>";
}
