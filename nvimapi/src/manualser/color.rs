use serde::Deserialize;
use suffixes::CastIt as _;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl From<u32> for Color {
    fn from(hex: u32) -> Self {
        let _alpha = (hex >> 24) & 0xFF;
        let red = (hex >> 16) & 0xFF;
        let green = (hex >> 8) & 0xFF;
        let blue = hex & 0xFF;
        return Color {
            r: red.u8(),
            g: green.u8(),
            b: blue.u8(),
        };
    }
}
impl From<i64> for Color {
    fn from(hex: i64) -> Self {
        Self::from(hex as u32)
    }
}
impl Default for Color {
    fn default() -> Self {
        Self { r: 255, g: 255, b: 255 }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        return deserializer.deserialize_i64(CVisitor);

        struct CVisitor;
        impl<'de> serde::de::Visitor<'de> for CVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting color")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
            {
                Ok(Color::from(v))
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
            {
                Ok(Color::from(v as u32))
            }
            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
            {
                Ok(Color::from(v))
            }
            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
            {
                Ok(Color::from(v as u32))
            }
            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
            {
                Ok(Color::from(v as u32))
            }
            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
            {
                Ok(Color::from(v as u32))
            }

        }


    }
}
