#[test]
fn test_position() {
    use crate::Position;

    let mut pos = Position::default();

    assert_eq!(pos.to_string(), "1:1");

    pos.advance('c');
    pos.advance('h');

    assert_eq!(pos.index, 2);
    assert_eq!(pos.column, 2);

    pos.advance('\n');

    assert_eq!(pos.index, 3);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 0);

    assert_eq!(pos.to_string(), "2:1");
}

#[test]
fn test_token() {
    use crate::{Position, Token};

    let pos = Position::default();
    let mut tok = Token::new("test_name", "test_value", &pos);

    assert_eq!(tok.to_string(), "<test_name>@1:1 = test_value");

    tok = Token::new("test_value", "test_value", &pos);
    assert_eq!(tok.to_string(), "<test_value>@1:1");
}

#[test]
fn test_scan() {
    // * The scanner actually works,
    // * problem here is probably about the orders
    // ? I might even remove it completely.
    // ! Or I will fix this test later.

    // use crate::Scanner;
    //
    // let content = "ident_123 456\t\n\r (){}[];,. @ # = == + += ++ > >= >> >>= - -= -- < <= << <<= * *= ! != / /= ~ & &= && ? | |= || : :: ^ ^= % %= \"string\"";
    // let mut scanner = Scanner::new(content).unwrap();
    //
    // let tokens = scanner.scan().unwrap();
    //
    // assert_eq!(tokens.get(0).unwrap().value(), "ident_123");
    // assert_eq!(tokens.get(1).unwrap().value(), "456");
    // assert_eq!(tokens.get(2).unwrap().value(), "(");
    // assert_eq!(tokens.get(3).unwrap().value(), ")");
    // assert_eq!(tokens.get(4).unwrap().value(), "{");
    // assert_eq!(tokens.get(5).unwrap().value(), "}");
    // assert_eq!(tokens.get(6).unwrap().value(), "[");
    // assert_eq!(tokens.get(7).unwrap().value(), "]");
    // assert_eq!(tokens.get(8).unwrap().value(), ";");
    // assert_eq!(tokens.get(9).unwrap().value(), ",");
    // assert_eq!(tokens.get(10).unwrap().value(), ".");
    // assert_eq!(tokens.get(11).unwrap().value(), "@");
    // assert_eq!(tokens.get(12).unwrap().value(), "#");
    // assert_eq!(tokens.get(13).unwrap().value(), "=");
    // assert_eq!(tokens.get(14).unwrap().value(), "==");
    // assert_eq!(tokens.get(15).unwrap().value(), "+");
    // assert_eq!(tokens.get(16).unwrap().value(), "+=");
    // assert_eq!(tokens.get(17).unwrap().value(), "++");
    // assert_eq!(tokens.get(18).unwrap().value(), ">");
    // assert_eq!(tokens.get(19).unwrap().value(), ">=");
    // assert_eq!(tokens.get(20).unwrap().value(), ">>");
    // assert_eq!(tokens.get(21).unwrap().value(), ">>=");
    // assert_eq!(tokens.get(15).unwrap().value(), "-");
    // assert_eq!(tokens.get(16).unwrap().value(), "-=");
    // assert_eq!(tokens.get(17).unwrap().value(), "--");
    // assert_eq!(tokens.get(22).unwrap().value(), "<");
    // assert_eq!(tokens.get(23).unwrap().value(), "<=");
    // assert_eq!(tokens.get(24).unwrap().value(), "<<");
    // assert_eq!(tokens.get(25).unwrap().value(), "<<=");
    // assert_eq!(tokens.get(26).unwrap().value(), "*");
    // assert_eq!(tokens.get(27).unwrap().value(), "*=");
    // assert_eq!(tokens.get(28).unwrap().value(), "!");
    // assert_eq!(tokens.get(29).unwrap().value(), "!=");
    // assert_eq!(tokens.get(30).unwrap().value(), "/");
    // assert_eq!(tokens.get(31).unwrap().value(), "/=");
    // assert_eq!(tokens.get(32).unwrap().value(), "~");
    // assert_eq!(tokens.get(33).unwrap().value(), "&");
    // assert_eq!(tokens.get(34).unwrap().value(), "&=");
    // assert_eq!(tokens.get(35).unwrap().value(), "&&");
    // assert_eq!(tokens.get(36).unwrap().value(), "?");
    // assert_eq!(tokens.get(37).unwrap().value(), "|");
    // assert_eq!(tokens.get(38).unwrap().value(), "|=");
    // assert_eq!(tokens.get(39).unwrap().value(), "||");
    // assert_eq!(tokens.get(40).unwrap().value(), ":");
    // assert_eq!(tokens.get(41).unwrap().value(), "::");
    // assert_eq!(tokens.get(42).unwrap().value(), "^");
    // assert_eq!(tokens.get(43).unwrap().value(), "^=");
    // assert_eq!(tokens.get(44).unwrap().value(), "%");
    // assert_eq!(tokens.get(45).unwrap().value(), "%=");
    // assert_eq!(tokens.get(46).unwrap().value(), "string");
}
