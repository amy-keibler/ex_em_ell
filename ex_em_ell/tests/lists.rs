#[derive(Debug, PartialEq, ex_em_ell::FromXmlDocument, ex_em_ell::ToXmlDocument)]
struct Example {
    children: Vec<ExampleChild>,
}

#[derive(
    Debug, PartialEq, ex_em_ell::FromXmlElement, ex_em_ell::ToXmlElement, ex_em_ell::NamedXmlElement,
)]
#[ex_em_ell(name = "child")]
struct ExampleChild {
    field: String,
}

#[test]
fn test_example_xmls() {
    insta::glob!("data/lists/valid_*.xml", |path| {
        let file = std::fs::File::open(path).expect(&format!("Failed to read file: {path:?}"));
        let example: Example = ex_em_ell::from_reader(&file)
            .expect(&format!("Failed to parse the XML file: {path:?}"));

        let round_trip = ex_em_ell::to_string_pretty(&example).expect("Failed to output XML");
        insta::assert_snapshot!(round_trip);
    });
}
