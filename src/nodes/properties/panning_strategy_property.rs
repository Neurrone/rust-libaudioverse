use super::super::super::libaudioverse_sys;
use super::super::super::{Error, Result};
use check;
use std::os::raw::c_int;

/// Indicates a strategy to use for panning. This is mostly for the multipanner node and the 3D components of this library.
#[repr(i32)]
pub enum PanningStrategy {
    /// Indicates HRTF panning.
    Hrtf = libaudioverse_sys::Lav_PANNING_STRATEGIES_Lav_PANNING_STRATEGY_HRTF,
    /// Indicates stereo panning.
    Stereo = libaudioverse_sys::Lav_PANNING_STRATEGIES_Lav_PANNING_STRATEGY_STEREO,
    /// Indicates 4.0 surround sound (quadraphonic) panning.
    Surround40 = libaudioverse_sys::Lav_PANNING_STRATEGIES_Lav_PANNING_STRATEGY_SURROUND40,
    /// Indicates 5.1 surround sound panning.
    Surround51 = libaudioverse_sys::Lav_PANNING_STRATEGIES_Lav_PANNING_STRATEGY_SURROUND51,
    /// Indicates 7.1 surround sound panning.
    Surround71 = libaudioverse_sys::Lav_PANNING_STRATEGIES_Lav_PANNING_STRATEGY_SURROUND71,
}

/// Proxy to a PanningStrategy property.
pub struct PanningStrategyProperty {
    pub(crate) index: c_int, // the index libaudioverse uses to identify this property for this node
    pub(crate) node_handle: libaudioverse_sys::LavHandle, // a handle to the parent node
}

impl PanningStrategyProperty {
    pub fn get(&self) -> Result<PanningStrategy> {
        let mut value: i32 = 0;
        check(unsafe {
            libaudioverse_sys::Lav_nodeGetIntProperty(self.node_handle, self.index, &mut value)
        })?;
        match value {
            libaudioverse_sys::Lav_PANNING_STRATEGIES_Lav_PANNING_STRATEGY_HRTF => {
                Ok(PanningStrategy::Hrtf)
            }
            libaudioverse_sys::Lav_PANNING_STRATEGIES_Lav_PANNING_STRATEGY_STEREO => {
                Ok(PanningStrategy::Stereo)
            }
            libaudioverse_sys::Lav_PANNING_STRATEGIES_Lav_PANNING_STRATEGY_SURROUND40 => {
                Ok(PanningStrategy::Surround40)
            }
            libaudioverse_sys::Lav_PANNING_STRATEGIES_Lav_PANNING_STRATEGY_SURROUND51 => {
                Ok(PanningStrategy::Surround51)
            }
            libaudioverse_sys::Lav_PANNING_STRATEGIES_Lav_PANNING_STRATEGY_SURROUND71 => {
                Ok(PanningStrategy::Surround71)
            }
            _ => Err(Error {
                code: libaudioverse_sys::Lav_ERRORS_Lav_ERROR_UNKNOWN,
                message: "Invalid panning strategy".to_string(),
            }),
        }
    }

    fn set_int(&self, value: i32) -> Result<()> {
        check(unsafe {
            libaudioverse_sys::Lav_nodeSetIntProperty(self.node_handle, self.index, value)
        })?;
        Ok(())
    }

    pub fn set(&self, strategy: PanningStrategy) -> Result<()> {
        self.set_int(strategy as i32)
    }
}
