use rmpv::Value;
use serde::Serialize;

pub trait ValueSeq {
    fn to_value(self) -> Value;
}
pub trait SerialSeq: Serialize {}
impl ValueSeq for Vec<Value> {
    fn to_value(self) -> Value {
        Value::Array(self)
    }
}
macro_rules! tuple_to_value {
    ($first: ident, $($params: ident),* $(,)?) => {
        impl<$first, $($params,)*> ValueSeq for ($first, $($params,)*)
        where
            Value: From<$first>,
            $(Value: From<$params>,)*
        {
            fn to_value(self) -> Value {
                #[allow(non_snake_case)]
                let ($first, $($params,)*) = self;
                let value = Value::Array(vec![
                    Value::from($first),
                    $(
                        Value::from($params),
                    )*
                ]);
                return value;
            }
        }
    };
}
impl ValueSeq for () {
    fn to_value(self) -> Value {
        Value::Array(Default::default())
    }
}
impl ValueSeq for [();0] {
    fn to_value(self) -> Value {
        Value::Array(Default::default())
    }
}
tuple_to_value!(V1,);
tuple_to_value!(V1, V2);
tuple_to_value!(V1, V2, V3);
tuple_to_value!(V1, V2, V3, V4);
tuple_to_value!(V1, V2, V3, V4, V5);
tuple_to_value!(V1, V2, V3, V4, V5, V6);
tuple_to_value!(V1, V2, V3, V4, V5, V6, V7);
tuple_to_value!(V1, V2, V3, V4, V5, V6, V7, V8);
tuple_to_value!(V1, V2, V3, V4, V5, V6, V7, V8, V9);
tuple_to_value!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10);
tuple_to_value!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11);
tuple_to_value!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12);
tuple_to_value!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13);
tuple_to_value!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14);
tuple_to_value!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15);
tuple_to_value!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16);
macro_rules! array_to_value {
    ($first: ty, $($params: literal),* $(,)?) => {
        $(
        impl ValueSeq for [$first; $params]
        where
            Value: From<$first>,
        {
            fn to_value(self) -> Value {
                Value::Array(
                    self.into_iter().map(Value::from).collect()
                )
            }
        }
        )*
    };
}
array_to_value!(u8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
array_to_value!(u16, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
array_to_value!(u32, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
array_to_value!(u64, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
array_to_value!(i8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
array_to_value!(i16, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
array_to_value!(i32, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
array_to_value!(i64, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
array_to_value!(String, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
array_to_value!(bool, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
array_to_value!(f32, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
array_to_value!(f64, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

mod check {
    use super::{ValueSeq, Value};
macro_rules! valueseq_for_single {
    ($($params: ty),* $(,)?) => {
        $(
        impl ValueSeq for $params
        where
            Value: From<$params>,
        {
            fn to_value(self) -> Value {
                Value::Array(vec![
                    Value::from(self)
                ])
            }
        }
        )*
    };
}
valueseq_for_single!(u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, String,);
// valueseq_for_single!(bool,);
}

macro_rules! serial_tuple {
    ($first: ident, $($params: ident),* $(,)?) => {
        impl<$first, $($params,)*> SerialSeq for ($first, $($params,)*)
        where
            $first: serde::Serialize,
            $($params: serde::Serialize,)*
        {}
    };
}
serial_tuple!(V1,);
serial_tuple!(V1, V2);
serial_tuple!(V1, V2, V3);
serial_tuple!(V1, V2, V3, V4);
serial_tuple!(V1, V2, V3, V4, V5);
serial_tuple!(V1, V2, V3, V4, V5, V6);
serial_tuple!(V1, V2, V3, V4, V5, V6, V7);
serial_tuple!(V1, V2, V3, V4, V5, V6, V7, V8);
serial_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9);
serial_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10);
serial_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11);
serial_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12);
serial_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13);
serial_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14);
serial_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15);
serial_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16);
impl<S: Serialize> SerialSeq for &[S] {}
impl<S: Serialize> SerialSeq for Vec<S> {}
impl<S: Serialize> SerialSeq for [S;0] {}
impl<S: Serialize> SerialSeq for [S;1] {}
impl<S: Serialize> SerialSeq for [S;2] {}
impl<S: Serialize> SerialSeq for [S;3] {}
impl<S: Serialize> SerialSeq for [S;4] {}
impl<S: Serialize> SerialSeq for [S;5] {}
impl<S: Serialize> SerialSeq for [S;6] {}
impl<S: Serialize> SerialSeq for [S;7] {}
impl<S: Serialize> SerialSeq for [S;8] {}
impl<S: Serialize> SerialSeq for [S;9] {}
impl<S: Serialize> SerialSeq for [S;10] {}
impl<S: Serialize> SerialSeq for [S;11] {}
impl<S: Serialize> SerialSeq for [S;12] {}
impl<S: Serialize> SerialSeq for [S;13] {}
impl<S: Serialize> SerialSeq for [S;14] {}
impl<S: Serialize> SerialSeq for [S;15] {}
impl<S: Serialize> SerialSeq for [S;16] {}
