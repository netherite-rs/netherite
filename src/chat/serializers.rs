use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::chat::component::Component;

pub trait ComponentSerializer<F, T> {
    type Error;

    fn serialize(component: &F) -> Result<T, Self::Error>;
    fn deserialize(t: &T) -> Result<F, Self::Error>;
}

pub struct JsonSerializer;

impl<T: Serialize + DeserializeOwned + Component> ComponentSerializer<T, String> for JsonSerializer {
    type Error = serde_json::Error;

    fn serialize(component: &T) -> Result<String, Self::Error> {
        serde_json::to_string(&component)
    }

    fn deserialize(t: &String) -> Result<T, Self::Error> {
        serde_json::from_str(&t)
    }
}