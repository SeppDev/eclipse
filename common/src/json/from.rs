use super::{Number, JSON};

impl From<bool> for JSON {
    fn from(value: bool) -> Self {
        JSON::Boolean(value)
    }
}

macro_rules! from_string {
    ($t:ty) => {
        impl From<$t> for JSON {
            fn from(value: $t) -> Self {
                JSON::String(value.to_string())
            }
        }
    };
}
from_string!(String);
from_string!(&str);

macro_rules! from_number {
    ($t:ty) => {
        impl From<$t> for Number {
            fn from(value: $t) -> Self {
                Number(value.to_string())
            }
        }

        impl From<$t> for JSON {
            fn from(value: $t) -> Self {
                JSON::Number(Number(value.to_string()))
            }
        }
    };
}

from_number!(isize);
from_number!(usize);
from_number!(i8);
from_number!(u8);
from_number!(i16);
from_number!(u16);
from_number!(i32);
from_number!(u32);
from_number!(i64);
from_number!(u64);
from_number!(f32);
from_number!(f64);
