/// capabilities implements the capability request functionality used to check
/// that a given capability has been granted for actions like zome calls
use crate::{
   signature::{Provenance, Signature},
};

use lib3h_persistence_api::{
    json::JsonString,
    cas::content::Address,
    error::PersistenceError
};
 
//--------------------------------------------------------------------------------------------------
// CapabilityRequest
//--------------------------------------------------------------------------------------------------

/// a struct to hold the capability information needed to make any capability request,
/// namely the provenance of the request (the agent address and signature) and the
/// actual token being used to make the request
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash, DefaultJson)]
pub struct CapabilityRequest {
    pub cap_token: Address,
    pub provenance: Provenance,
}

impl CapabilityRequest {
    pub fn new(token: Address, requester: Address, signature: Signature) -> Self {
        CapabilityRequest {
            cap_token: token,
            provenance: Provenance::new(requester, signature),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use lib3h_persistence_api::cas::content::Address;

    #[test]
    fn test_capability_request_new() {
        let cap_call = CapabilityRequest::new(
            Address::from("123"),
            Address::from("requester"),
            Signature::fake(),
        );
        assert_eq!(
            CapabilityRequest {
                cap_token: Address::from("123"),
                provenance: Provenance::new(Address::from("requester"), Signature::fake()),
            },
            cap_call
        );
    }
}
