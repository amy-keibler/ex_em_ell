#[derive(Debug, PartialEq, ex_em_ell::ToXmlDocument)]
struct Example {
    field: String,
    child: ExampleChild,
}

#[derive(Debug, PartialEq, ex_em_ell::ToXmlElement)]
struct ExampleChild {
    field: String,
}

#[test]
fn test_from_xml() {
    let input = "<example><field>value</field><child><field>value</field></child></example>";
    let expected = Example {
        field: "value".to_string(),
        child: ExampleChild {
            field: "value".to_string(),
        },
    };
}

#[test]
fn test_to_xml() {
    let input = Example {
        field: "value".to_string(),
        child: ExampleChild {
            field: "value".to_string(),
        },
    };
    let actual = ex_em_ell::to_string_pretty(&input).expect("Failed to output XML");
    insta::assert_snapshot!(actual);
}
