use crate::Foam;
use crate::FoamElement;

#[test]
fn attribution() {
    let text = "a_var value;";
    let result = Foam::try_from(text).unwrap();
    let expected = vec![FoamElement::Attribution(
        String::from("a_var"),
        vec![FoamElement::Element(String::from("value"))],
    )];
    assert_eq!(result.as_ref(), &expected);
}

#[test]
fn multiple_attribution() {
    let text = "a_var value1 value2 value3;";
    let result = Foam::try_from(text).unwrap();
    let expected = vec![FoamElement::Attribution(
        String::from("a_var"),
        vec![
            FoamElement::Element(String::from("value1")),
            FoamElement::Element(String::from("value2")),
            FoamElement::Element(String::from("value3")),
        ],
    )];
    assert_eq!(result.as_ref(), &expected);
}

#[test]
fn two_attributions() {
    let text = "var1 value1; var2 value2;";
    let result = Foam::try_from(text).unwrap();
    let expected = vec![
        FoamElement::Attribution(
            String::from("var1"),
            vec![FoamElement::Element(String::from("value1"))],
        ),
        FoamElement::Attribution(
            String::from("var2"),
            vec![FoamElement::Element(String::from("value2"))],
        ),
    ];
    assert_eq!(result.as_ref(), &expected);
}
