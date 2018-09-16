use super::super::super::libaudioverse_sys;
use super::super::super::{Error, Result};
use check;
use std::os::raw::c_int;

/// used in the 3D components of this library. Indicates how sound should become quieter as objects move away from the listener.
/// Libaudioverse computes the distance from the center of the source to the listener, then subtracts the size. In the following, distance refers to this distance. Libaudioverse also computes a second value called distancePercent; specifically, distancePercent = distance/maxDistance.
#[repr(i32)]
pub enum DistanceModel {
    /// Sounds fall off as 1/(1+315*distancePercent). Just before maxDistance, the gain of the sound will be approximately -25 DB. This distance model emphasizes distance changes when sounds are close, but treats distance changes of further away sources more subtly. For full benefit of the effect of close sounds, this distance model must be used with fairly large values for maxDistance, usually around 300.
    Inverse = libaudioverse_sys::Lav_DISTANCE_MODELS_Lav_DISTANCE_MODEL_INVERSE,
    /// Sounds fall off as 1.0/(1+315*distancePercent*distancePercent). This is a standard inverse square law, modified such that the sound volume just before maxDistance is about -25 DB. Of the available distance models, this is the closest to an accurate simulation of large, wide-open places such as fields and stadiums.
    InverseSquare = libaudioverse_sys::Lav_DISTANCE_MODELS_Lav_DISTANCE_MODEL_INVERSE_SQUARE,
    /// Sound falls off as 1-distancePercent.
    Linear = libaudioverse_sys::Lav_DISTANCE_MODELS_Lav_DISTANCE_MODEL_LINEAR,
}

/// Proxy to a DistanceModel property.
pub struct DistanceModelProperty {
    pub(crate) index: c_int, // the index libaudioverse uses to identify this property for this node
    pub(crate) node_handle: libaudioverse_sys::LavHandle, // a handle to the parent node
}

impl DistanceModelProperty {
    pub fn get(&self) -> Result<DistanceModel> {
        let mut value: i32 = 0;
        check(unsafe {
            libaudioverse_sys::Lav_nodeGetIntProperty(self.node_handle, self.index, &mut value)
        })?;
        match value {
            libaudioverse_sys::Lav_DISTANCE_MODELS_Lav_DISTANCE_MODEL_INVERSE => {
                Ok(DistanceModel::Inverse)
            }
            libaudioverse_sys::Lav_DISTANCE_MODELS_Lav_DISTANCE_MODEL_INVERSE_SQUARE => {
                Ok(DistanceModel::InverseSquare)
            }
            libaudioverse_sys::Lav_DISTANCE_MODELS_Lav_DISTANCE_MODEL_LINEAR => {
                Ok(DistanceModel::Linear)
            }
            _ => Err(Error {
                code: libaudioverse_sys::Lav_ERRORS_Lav_ERROR_UNKNOWN,
                message: "Invalid distance model".to_string(),
            }),
        }
    }

    fn set_int(&self, value: i32) -> Result<()> {
        check(unsafe {
            libaudioverse_sys::Lav_nodeSetIntProperty(self.node_handle, self.index, value)
        })?;
        Ok(())
    }

    pub fn set(&self, distance_model: DistanceModel) -> Result<()> {
        self.set_int(distance_model as i32)
    }

    /*
    pub fn inverse(&self) -> Result<()> {
        self.set_int(DistanceModel::Inverse as i32)
    }
    
    pub fn inverse_square(&self) -> Result<()> {
        self.set_int(DistanceModel::InverseSquare as i32)
    }
    
    pub fn linear(&self) -> Result<()> {
        self.set_int(DistanceModel::Linear as i32)
    }
    */
}
