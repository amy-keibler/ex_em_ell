#[derive(Debug, PartialEq, ex_em_ell::FromXmlDocument, ex_em_ell::ToXmlDocument)]
struct Example {
    // Support Acronym Case conventions without forcing the Rust variable to be field_u_r_l
    #[ex_em_ell(rename = "fieldURL")]
    field_url: String,

    #[ex_em_ell(rename = "childURL")]
    child_url: ExampleChild,
}

#[derive(Debug, PartialEq, ex_em_ell::FromXmlElement, ex_em_ell::ToXmlElement)]
struct ExampleChild {
    #[ex_em_ell(rename = "fieldURL")]
    field_url: String,
}

#[test]
fn test_example_xmls() {
    insta::glob!("data/rename/rename_*.xml", |path| {
        let file = std::fs::File::open(path).expect(&format!("Failed to read file: {path:?}"));
        let example: Example = ex_em_ell::from_reader(&file)
            .expect(&format!("Failed to parse the XML file: {path:?}"));

        let round_trip = ex_em_ell::to_string_pretty(&example).expect("Failed to output XML");
        insta::assert_snapshot!(round_trip);
    });
}
