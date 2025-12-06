use rmpv::Value;
use serde::Deserialize;

use crate::nvimapi::UiEvent;
#[derive(Debug)]
pub enum Notification {
    Redraw(Vec<UiEvent>),
    Unknown(Box<(String, Value)>),
}

impl Notification {
    pub fn name(&self) -> & str {
        match self {
            Notification::Redraw(_) => "redraw",
            Notification::Unknown(u) => &u.0,
        }
    }
}

impl<'de> Deserialize<'de> for Notification {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        return deserializer.deserialize_seq(NVisitor);

        use serde::de::Error as DError;
        struct NVisitor;
        impl<'de> serde::de::Visitor<'de> for NVisitor {
            type Value = Notification;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("seq containing nvim notification.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let msg = "missing, expected 2 items";
                let Some(name) = seq.next_element::<String>()? else { return Err(DError::custom(msg)) };
                if name != "redraw" {
                    let Some(unknown) = seq.next_element::<Value>()? else { return Err(DError::custom(msg)) };
                    return Ok(Notification::Unknown(Box::new((name, unknown))));
                }
                //name is redraw
                let Some(ui_events) = seq.next_element::<Vec<UiEvent>>()? else { return Err(DError::custom(msg)) };
                return Ok(Notification::Redraw(ui_events));
            }
        }

    }
}
