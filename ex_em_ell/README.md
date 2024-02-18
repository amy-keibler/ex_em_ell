# `ex_em_ell`

`ex_em_ell` is an opinionated XML library based on the excellent
[`xml-rs`](https://crates.io/crates/xml-rs) library. It provides to/from traits
and derive macros to reduce boilerplate required when using the underlying
library.

## Project Status

This project is currently in an alpha state and the API is expected to go
through rounds of iteration before reaching a `1.x.y` release. In its current
state, it is being developed based on the needs of specific features for
projects, rather than based on a specific roadmap.

## Purpose

As mentioned, `ex_eml_ell` is an opinionated library, that resulted in none of
the other libraries quite fitting a particular niche usecase. If those libraries
meet your usecase, they are likely much more polished and refined compared to
this project, so we would recommend them instead.

### Fallible Parsing

At the time of writing this library, many of the parsers that would otherwise
fit our usecase required your types to implement `Default`, and would
automatically populate the instance of your type with default values. We need a
library that is capable of:

- Failing when a required field is not provided
- Resulting in an error message that can be used to trace the exact path through
  the XML structure to what field was missing (e.g. `data > components > 1 >
  component is missing the "name" field` is significantly more helpful to a user
  than `expected "name" field`)

### Not Built On Serde

As XML is a bit outside of the common patterns of `serde`'s design, libraries
that attempt to integrate with `serde` use things such as `#[serde(rename =
"@attribute")` to specify the specific XML structure to use for reading/writing
documents. This is useful for situations when a type is only going to be used
for XML, but our usecase required the ability for a type to represent parsing
both XML and JSON documents. We use `serde` for JSON and this library for `XML`.

## Usage

The crate provides traits and corresponding derive macros. The main ones worth
mentioning are `FromXmlDocument`/`ToXmlDocument` and
`FromXmlElement`/`ToXmlElement`. A struct with the following definition

```rust
#[derive(ex_em_ell::FromXmlDocument, ex_em_ell::ToXmlDocument)]
struct Example {
    #[ex_em_ell(rename = "type")]
    example_type: String,

    child: ExampleChild,
}

#[derive(ex_em_ell::FromXmlElement, ex_em_ell::ToXmlElement)]
struct ExampleChild {
    field: String,
}
```

would correspond to the following XML

``` xml
<example>
  <type>example value</type>
  <child>
    <field>example field value</field>
  </child>
</example>
```

## License

This project is dual-licensed under the terms of the
[MIT](https://spdx.org/licenses/MIT.html) license or the
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) license. This was
chosen for compatibility with the broader Rust community.
