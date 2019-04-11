use cita_types::{Address, U256};
use libproto::request::Estimate;
use util::Bytes;

/// Estimate request
#[derive(Debug, Default, PartialEq)]
pub struct EstimateRequest {
    /// From
    pub from: Option<Address>,
    /// To
    pub to: Option<Address>,
    /// value
    pub value: Option<U256>,
    /// Data
    pub data: Option<Bytes>,
}

impl From<Estimate> for EstimateRequest {
    fn from(est: Estimate) -> Self {
        EstimateRequest {
            from: if est.get_from().is_empty() {
                None
            } else {
                Some(Address::from(est.get_from()))
            },
            to: if est.get_to().is_empty() {
                None
            } else {
                Some(Address::from(est.get_to()))
            },
            value: if est.get_value().is_empty() {
                None
            } else {
                Some(U256::from(est.get_value()))
            },
            data: if est.data.is_empty() {
                None
            } else {
                Some(est.data)
            },
        }
    }
}

#[cfg(test)]
mod tests {}
