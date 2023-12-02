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
