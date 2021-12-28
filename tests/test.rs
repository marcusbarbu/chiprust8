#[test]
fn foo() {
    let a: u8 = 0xF1;
    let b: u8 = 0x10;
    let c: u8 = a.wrapping_add(b);
    assert_eq!(c, 1);
}