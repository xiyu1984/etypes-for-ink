
use scale::{
    Encode,
    Decode,
};

#[derive(Hash, Encode, Decode, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ::ink::storage::traits::StorageLayout)
)]
pub struct UDData {
    pub name: ink::prelude::string::String,
    pub value: u128,
    pub uri: ink::prelude::string::String,
}

impl UDData {
    pub fn new(name: ink::prelude::string::String, value: u128, uri: ink::prelude::string::String)-> Self {
        UDData { name, value, uri }
    }
}
