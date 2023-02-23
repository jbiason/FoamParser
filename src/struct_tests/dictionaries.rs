use crate::Foam;
use crate::FoamElement;

#[test]
fn single_value() {
    let text = "dict { a 1; }";
    let result = Foam::try_from(text).unwrap();
    let expected = vec![FoamElement::Dict(
        String::from("dict"),
        vec![FoamElement::Attribution(
            String::from("a"),
            vec![FoamElement::Element(String::from("1"))],
        )],
    )];
    assert_eq!(result.as_ref(), &expected);
}

#[test]
fn with_lists() {
    let text = "dict { list (1 2); }";
    let result = Foam::try_from(text).unwrap();
    let expected = vec![FoamElement::Dict(
        String::from("dict"),
        vec![FoamElement::Attribution(
            String::from("list"),
            vec![FoamElement::List(vec![
                FoamElement::Element(String::from("1")),
                FoamElement::Element(String::from("2")),
            ])],
        )],
    )];
    assert_eq!(result.as_ref(), &expected);
}

#[test]
fn two_values() {
    let text = "dict { a 1; b 2; }";
    let result = Foam::try_from(text).unwrap();
    let expected = vec![FoamElement::Dict(
        String::from("dict"),
        vec![
            FoamElement::Attribution(
                String::from("a"),
                vec![FoamElement::Element(String::from("1"))],
            ),
            FoamElement::Attribution(
                String::from("b"),
                vec![FoamElement::Element(String::from("2"))],
            ),
        ],
    )];
    assert_eq!(result.as_ref(), &expected);
}
