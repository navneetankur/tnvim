use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use rmpv::Value;
use serde::{Deserialize, Deserializer, Serialize};
use crate::{error, TryFromValue};

/// a vec of tuple of elements,
/// pretending to be a map for serilization purpose.
/// Will NOT avoid duplicates.
#[derive(Default, Debug)]
pub struct Pairs<K = Value, V = K> {
    inner: Vec<(K, V)>,
}
impl<K, V> Pairs<K, V> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }
}
impl<K, V> Pairs<K, V>
where 
    K: PartialEq
{
    pub fn get_for_key(&self, key: &K) -> Option<&V> {
        for (k,v) in self.iter() {
            if k == key { return Some(v) }
        }
        return None;
    }
}
impl<K, V> Deref for Pairs<K, V> {
    type Target = Vec<(K, V)>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<K, V> DerefMut for Pairs<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl<K, V> From<Vec<(K, V)>> for Pairs<K, V> {
    fn from(inner: Vec<(K, V)>) -> Self {
        Self { inner }
    }
}
impl<K, V> From<Pairs<K, V>> for Vec<(K, V)> {
    fn from(value: Pairs<K, V>) -> Self {
        value.inner
    }
}
impl<K, V> TryFromValue for Pairs<K, V>
where
    K: TryFromValue,
    V: TryFromValue,
{
    fn try_from_value(value: Value) -> error::Result<Self>
    where
        Self: Sized,
    {
        let Value::Map(value) = value else {
            return error::with_msg("expecting map");
        };
        let mut rv = Vec::with_capacity(value.len());
        for (k, v) in value {
            let key = K::try_from_value(k)?;
            let value = V::try_from_value(v)?;
            rv.push((key, value));
        }
        return Ok(Self { inner: rv });
    }
}

impl<'de, K, V> Deserialize<'de> for Pairs<K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = MapVisitor {
            marker: PhantomData,
        };
        return deserializer.deserialize_map(visitor);

        struct MapVisitor<K, V> {
            marker: PhantomData<Pairs<K, V>>,
        }

        impl<'de, K, V> serde::de::Visitor<'de> for MapVisitor<K, V>
        where
            K: Deserialize<'de>,
            V: Deserialize<'de>,
        {
            type Value = Pairs<K, V>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a map")
            }

            #[inline]
            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut values = Pairs::with_capacity(access.size_hint().unwrap_or(0));
                while let Some((key, value)) = access.next_entry()? {
                    values.push((key, value));
                }
                Ok(values)
            }
        }
    }
}

impl<K, V> Serialize for Pairs<K, V>
where
    K: Serialize,
    V: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_map(self.iter().map(|(k,v)|(k,v)))
    }
}
impl<K,V> IntoIterator for Pairs<K,V> {
    type Item = (K,V);

    type IntoIter = <Vec<(K, V)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
impl<'a, K,V> IntoIterator for &'a Pairs<K,V> {
    // type Item = &'a (K,V);
    type Item = <&'a Vec<(K, V)> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<(K, V)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
impl<'a, K,V> IntoIterator for &'a mut Pairs<K,V> {
    type Item = &'a mut (K,V);
    type IntoIter = <&'a mut Vec<(K, V)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}
// todo, make Pairs<Value> faster when
// specialization arrives.
impl<K,V> From<Pairs<K,V>> for Value
where 
    Value: From<K>,
    Value: From<V>,
{
    fn from(pairs: Pairs<K,V>) -> Self {
        let mut rv = Vec::with_capacity(pairs.len());
        for (k,v) in pairs {
            let k = Value::from(k);
            let v = Value::from(v);
            rv.push((k,v));
        }
        return Value::Map(rv);
    }
}
