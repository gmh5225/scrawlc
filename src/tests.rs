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
