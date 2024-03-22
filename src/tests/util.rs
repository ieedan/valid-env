use crate::util;

#[test]
fn number_pad_pads_correctly() {
    let expected = "  1";
    let min = 3;
    let num = 1;

    let result = util::number_pad(num, min);

    assert_eq!(expected, result);
}

#[test]
fn trims_quotes_trims_quotes() {
    let expected = "hello";
    let input = "\"hello\"";

    let result = util::trim_quotes(input);

    assert_eq!(expected, result);
}

#[test]
fn trims_quotes_returns_same() {
    let expected = "hello";
    let input = "hello";

    let result = util::trim_quotes(input);

    assert_eq!(expected, result);
}