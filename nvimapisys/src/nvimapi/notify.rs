use log::debug;
use rmpv::Value;
use serde::Deserialize;

use crate::nvimapi::UiEvent;
#[derive(Debug)]
pub enum Notify {
    Redraw(Vec<UiEvent>),
    Unknown(String, Value),
}

impl<'de> Deserialize<'de> for Notify {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        return deserializer.deserialize_seq(NVisitor);

        use serde::de::Error as DError;
        struct NVisitor;
        impl<'de> serde::de::Visitor<'de> for NVisitor {
            type Value = Notify;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("seq containing nvim notification.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                debug!("Notify visitor");
                let msg = "missing, expected 2 items";
                let Some(name) = seq.next_element::<String>()? else { return Err(DError::custom(msg)) };
                debug!("name is: {name}");
                if name != "redraw" {
                    let Some(unknown) = seq.next_element::<Value>()? else { return Err(DError::custom(msg)) };
                    return Ok(Notify::Unknown(name, unknown));
                }
                //name is redraw
                debug!("ready to get after redraw");
                let Some(ui_events) = seq.next_element::<Vec<UiEvent>>()? else { return Err(DError::custom(msg)) };
                return Ok(Notify::Redraw(ui_events));
            }
        }

    }
}
