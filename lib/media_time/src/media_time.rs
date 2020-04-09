use fraction::Fraction;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MediaTimeError {
    #[error("missing numerator in timebase")]
    TimebaseNumeratorMissing,
    #[error("missing denominator in timebase")]
    TimebaseDenominatorMissing,
    #[error("invalid denominator in timebase")]
    TimebaseDenominatorInvalid,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MediaTime(time::Duration);

impl MediaTime {
    pub fn from_rational(timestamp: i64, base: &Fraction) -> Result<MediaTime, MediaTimeError> {
        let num: u64 = *base.numer().ok_or(MediaTimeError::TimebaseNumeratorMissing)?;
        let den: u64 = *base.denom().ok_or(MediaTimeError::TimebaseDenominatorMissing)?;
        if den == 0 {
            return Err(MediaTimeError::TimebaseDenominatorInvalid)
        }

        Ok(MediaTime(time::Duration::milliseconds(
            (1000 * timestamp as i128 * num as i128 / den as i128) as i64,
        )))
    }

    #[inline(always)]
    pub fn from_millis(timestamp: i64) -> MediaTime {
        MediaTime(time::Duration::milliseconds(timestamp))
    }

    #[inline(always)]
    pub fn from_seconds(timestamp: i64) -> MediaTime {
        MediaTime(time::Duration::seconds(timestamp))
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[inline(always)]
    pub fn seconds(&self) -> i64 {
        self.0.whole_seconds()
    }
}

impl std::fmt::Display for MediaTime {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let z = self.0.subsec_milliseconds();
        let s = self.0.whole_seconds() % 60;
        let m = self.0.whole_seconds() / 60 % 60;
        let h = self.0.whole_seconds() / 3600;

        if h == 0 {
            write!(f, "{:02}:{:02}.{:03}", m, s, z)
        } else {
            write!(f, "{:02}:{:02}:{:02}.{:03}", h, m, s, z)
        }
    }
}

impl std::ops::Add for MediaTime {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for MediaTime {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}
