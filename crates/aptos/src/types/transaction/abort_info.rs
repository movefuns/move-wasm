use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
//#[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
//#[cfg_attr(any(test, feature = "fuzzing"), proptest(no_params))]
pub struct AbortInfo {
    pub reason_name: String,
    pub description: String,
}
