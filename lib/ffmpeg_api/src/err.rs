use num_traits::FromPrimitive;
use thiserror::Error;

use crate::err_ffi::AvFfiError;
use crate::err_av::AvInternalError;

#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum AVError {
    #[error(transparent)]
    Internal(AvInternalError),
    #[error(transparent)]
    System(AvFfiError),
    #[error("Unknown error occured: {0}")]
    Unknown(i32),
}

impl AVError {
    pub fn from_errno(errno: i32) -> Result<(), AVError> {
        match errno {
            0 => Ok(()),
            errno => Err(AVError::from(-errno)),
        }
    }
}

impl From<i32> for AVError {
    fn from(value: i32) -> Self {
        if let Some(error) = AvFfiError::from_i32(value) {
            return AVError::System(error);
        }
        if let Some(error) = AvInternalError::from_i32(value) {
            return AVError::Internal(error);
        }
        return AVError::Unknown(value);
    }
}