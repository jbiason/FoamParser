use crate::Foam;
use crate::FoamElement;

#[test]
fn a_list() {
    let text = "var ( 1 2 );";
    let result = Foam::try_from(text).unwrap();
    let expected = vec![FoamElement::Attribution(
        String::from("var"),
        vec![FoamElement::List(vec![
            FoamElement::Element(String::from("1")),
            FoamElement::Element(String::from("2")),
        ])],
    )];
    assert_eq!(result.as_ref(), &expected);
}

#[test]
fn list_of_lists() {
    let text = "var ((1 2) (4 5));";
    let result = Foam::try_from(text).unwrap();
    let expected = vec![FoamElement::Attribution(
        String::from("var"),
        vec![FoamElement::List(vec![
            FoamElement::List(vec![
                FoamElement::Element(String::from("1")),
                FoamElement::Element(String::from("2")),
            ]),
            FoamElement::List(vec![
                FoamElement::Element(String::from("4")),
                FoamElement::Element(String::from("5")),
            ]),
        ])],
    )];
    assert_eq!(result.as_ref(), &expected);
}
