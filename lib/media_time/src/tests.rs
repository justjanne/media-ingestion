use crate::MediaTime;
use fraction::Fraction;

#[test]
fn from_millis_works() {
    assert_eq!(MediaTime::from_millis(0).seconds(), 0);
    assert_eq!(MediaTime::from_millis(1357).seconds(), 1);
    assert_eq!(MediaTime::from_millis(1357).milliseconds(), 1357);
}

#[test]
fn from_seconds_works() {
    assert_eq!(MediaTime::from_seconds(0).seconds(), 0);
    assert_eq!(MediaTime::from_seconds(1357).seconds(), 1357);
    assert_eq!(MediaTime::from_seconds(1357).milliseconds(), 1357000);
}

#[test]
fn from_rational_works() {
    assert_eq!(MediaTime::from_rational(0, &Fraction::new(1u64, 1u64)).unwrap().seconds(), 0);
    assert_eq!(MediaTime::from_rational(1357, &Fraction::new(1u64, 1u64)).unwrap().seconds(), 1357);
    assert_eq!(MediaTime::from_rational(30, &Fraction::new(1u64, 3u64)).unwrap().seconds(), 10);
}
