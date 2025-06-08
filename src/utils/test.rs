use crate::utils::crop;

#[test]
fn crop_i8_within_range() {
    let value = crop::<i8>(-127, 0);

    assert_eq!(-127, value)
}

#[test]
fn crop_i8_exceed_range() {
    let value = crop::<i8>(11, 10);

    assert_eq!(10, value)
}

#[test]
fn crop_u8_within_range() {
    let value = crop::<u8>(127, 128);

    assert_eq!(127, value)
}

#[test]
fn crop_u8_exceed_range() {
    let value = crop::<u8>(129, 128);

    assert_eq!(128, value)
}
