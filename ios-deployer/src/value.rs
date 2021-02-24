use std::collections::HashMap as Map;
use std::fmt;
use std::mem;

use core_foundation::{
    array::CFArray, base::CFType, base::FromVoid, base::ItemRef, base::TCFType, boolean::CFBoolean,
    dictionary::CFDictionary, dictionary::CFMutableDictionary, number::CFNumber, string::CFString,
};

/// A type that can be used to index into a `serde_json::Value`.
///
/// The [`get`] and [`get_mut`] methods of `Value` accept any type that
/// implements `Index`, as does the [square-bracket indexing operator]. This
/// trait is implemented for strings which are used as the index into a JSON
/// map, and for `usize` which is used as the index into a JSON array.
///
/// [`get`]: ../enum.Value.html#method.get
/// [`get_mut`]: ../enum.Value.html#method.get_mut
/// [square-bracket indexing operator]: ../enum.Value.html#impl-Index%3CI%3E
///
/// This trait is sealed and cannot be implemented for types outside of
/// `serde_json`.
///
/// # Examples
///
/// ```
/// # use serde_json::json;
/// #
/// let data = json!({ "inner": [1, 2, 3] });
///
/// // Data is a JSON map so it can be indexed with a string.
/// let inner = &data["inner"];
///
/// // Inner is a JSON array so it can be indexed with an integer.
/// let first = &inner[0];
///
/// assert_eq!(first, 1);
/// ```
pub trait Index: private::Sealed {
    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;

    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value>;

    /// Panic if array index out of bounds. If key is not already in the object,
    /// insert it with a value of null. Panic if Value is a type that cannot be
    /// indexed into, except if Value is null then it can be treated as an empty
    /// object.
    #[doc(hidden)]
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value;
}

impl Index for usize {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match *v {
            Value::Array(ref vec) => vec.get(*self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match *v {
            Value::Array(ref mut vec) => vec.get_mut(*self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        match *v {
            Value::Array(ref mut vec) => {
                let len = vec.len();
                vec.get_mut(*self).unwrap_or_else(|| {
                    panic!("cannot access index {} of array of length {}", self, len)
                })
            }
            _ => panic!("cannot access index {} of {:?}", self, v),
        }
    }
}

impl Index for str {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match *v {
            Value::Dictionary(ref map) => map.get(self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match *v {
            Value::Dictionary(ref mut map) => map.get_mut(self),
            _ => None,
        }
    }

    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        if let Value::Null = *v {
            *v = Value::Dictionary(Map::new());
        }
        match *v {
            Value::Dictionary(ref mut map) => map.entry(self.to_owned()).or_insert(Value::Null),
            _ => panic!("cannot access key {:?} in {:?}", self, v),
        }
    }
}

impl Index for String {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        self[..].index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        self[..].index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        self[..].index_or_insert(v)
    }
}

impl<'a, T> Index for &'a T
where
    T: ?Sized + Index,
{
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        (**self).index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        (**self).index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        (**self).index_or_insert(v)
    }
}

// Prevent users from implementing the Index trait.
mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl<'a, T> Sealed for &'a T where T: ?Sized + Sealed {}
}

pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Dictionary(Map<String, Value>),
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<ItemRef<'_, CFType>> for Value {
    fn from(itemref: ItemRef<'_, CFType>) -> Self {
        (*itemref).as_CFType().into()
    }
}

impl From<CFType> for Value {
    fn from(cftype: CFType) -> Self {
        if cftype.instance_of::<CFString>() {
            Value::String(cftype.downcast::<CFString>().unwrap().to_string())
        } else if cftype.instance_of::<CFArray>() {
            unimplemented!();
            /*
            Value::Array(
                cftype
                    .downcast::<CFArray>()
                    .unwrap()
                    .iter()
                    .map(|v| v.into())
                    .collect::<Vec<_>>(),
            )
            */
        } else if cftype.instance_of::<CFBoolean>() {
            Value::Bool(cftype.downcast::<CFBoolean>().unwrap().into())
        } else if cftype.instance_of::<CFNumber>() {
            let number = cftype.downcast::<CFNumber>().unwrap();
            match number.to_i32() {
                Some(v) => return v.into(),
                None => {}
            }
            match number.to_i64() {
                Some(v) => return v.into(),
                None => {}
            }
            match number.to_f32() {
                Some(v) => return v.into(),
                None => {}
            }
            match number.to_f64() {
                Some(v) => return v.into(),
                None => {
                    unreachable!()
                }
            }
        } else if cftype.instance_of::<CFDictionary>() {
            unimplemented!();
        } else {
            unimplemented!();
        }
    }
}

impl Value {
    fn to_cfstring(&self) -> Option<CFString> {
        match *self {
            Value::String(ref s) => Some(s.as_str().into()),
            _ => None,
        }
    }

    fn to_cfnumber(&self) -> Option<CFNumber> {
        match *self {
            Value::Number(ref num) => {
                if num.is_f32() {
                    Some(num.as_f32().unwrap().into())
                } else if num.is_f64() {
                    Some(num.as_f64().unwrap().into())
                } else if num.is_i32() {
                    Some(num.as_i32().unwrap().into())
                } else if num.is_i64() {
                    Some(num.as_i64().unwrap().into())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn to_cfarray(&self) -> Option<CFArray<Value>> {
        None
    }

    fn to_cfbool(&self) -> Option<CFBoolean> {
        None
    }

    fn to_cfdictionary(&self) -> Option<CFDictionary> {
        None
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Null => formatter.debug_tuple("Null").finish(),
            Value::Bool(v) => formatter.debug_tuple("Bool").field(&v).finish(),
            Value::Number(ref v) => fmt::Debug::fmt(v, formatter),
            Value::String(ref v) => formatter.debug_tuple("String").field(v).finish(),
            Value::Array(ref v) => {
                formatter.write_str("Array(")?;
                fmt::Debug::fmt(v, formatter)?;
                formatter.write_str(")")
            }
            Value::Dictionary(ref v) => {
                formatter.write_str("Dictionary(")?;
                fmt::Debug::fmt(v, formatter)?;
                formatter.write_str(")")
            }
        }
    }
}

impl Value {
    /// Index into a JSON array or map. A string index can be used to access a
    /// value in a map, and a usize index can be used to access an element of an
    /// array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let object = json!({ "A": 65, "B": 66, "C": 67 });
    /// assert_eq!(*object.get("A").unwrap(), json!(65));
    ///
    /// let array = json!([ "A", "B", "C" ]);
    /// assert_eq!(*array.get(2).unwrap(), json!("C"));
    ///
    /// assert_eq!(array.get("A"), None);
    /// ```
    ///
    /// Square brackets can also be used to index into a value in a more concise
    /// way. This returns `Value::Null` in cases where `get` would have returned
    /// `None`.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let object = json!({
    ///     "A": ["a", "á", "à"],
    ///     "B": ["b", "b́"],
    ///     "C": ["c", "ć", "ć̣", "ḉ"],
    /// });
    /// assert_eq!(object["B"][0], json!("b"));
    ///
    /// assert_eq!(object["D"], json!(null));
    /// assert_eq!(object[0]["x"]["y"]["z"], json!(null));
    /// ```
    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }

    /// Mutably index into a JSON array or map. A string index can be used to
    /// access a value in a map, and a usize index can be used to access an
    /// element of an array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut object = json!({ "A": 65, "B": 66, "C": 67 });
    /// *object.get_mut("A").unwrap() = json!(69);
    ///
    /// let mut array = json!([ "A", "B", "C" ]);
    /// *array.get_mut(2).unwrap() = json!("D");
    /// ```
    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {
        index.index_into_mut(self)
    }

    /// Returns true if the `Value` is an Object. Returns false otherwise.
    ///
    /// For any Value on which `is_object` returns true, `as_object` and
    /// `as_object_mut` are guaranteed to return the map representation of the
    /// object.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let obj = json!({ "a": { "nested": true }, "b": ["an", "array"] });
    ///
    /// assert!(obj.is_object());
    /// assert!(obj["a"].is_object());
    ///
    /// // array, not an object
    /// assert!(!obj["b"].is_object());
    /// ```
    pub fn is_dictionary(&self) -> bool {
        self.as_dictionary().is_some()
    }

    /// If the `Value` is an Object, returns the associated Map. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": { "nested": true }, "b": ["an", "array"] });
    ///
    /// // The length of `{"nested": true}` is 1 entry.
    /// assert_eq!(v["a"].as_object().unwrap().len(), 1);
    ///
    /// // The array `["an", "array"]` is not an object.
    /// assert_eq!(v["b"].as_object(), None);
    /// ```
    pub fn as_dictionary(&self) -> Option<&Map<String, Value>> {
        match *self {
            Value::Dictionary(ref map) => Some(map),
            _ => None,
        }
    }

    /// If the `Value` is an Object, returns the associated mutable Map.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut v = json!({ "a": { "nested": true } });
    ///
    /// v["a"].as_object_mut().unwrap().clear();
    /// assert_eq!(v, json!({ "a": {} }));
    /// ```
    pub fn as_dictionary_mut(&mut self) -> Option<&mut Map<String, Value>> {
        match *self {
            Value::Dictionary(ref mut map) => Some(map),
            _ => None,
        }
    }

    /// Returns true if the `Value` is an Array. Returns false otherwise.
    ///
    /// For any Value on which `is_array` returns true, `as_array` and
    /// `as_array_mut` are guaranteed to return the vector representing the
    /// array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let obj = json!({ "a": ["an", "array"], "b": { "an": "object" } });
    ///
    /// assert!(obj["a"].is_array());
    ///
    /// // an object, not an array
    /// assert!(!obj["b"].is_array());
    /// ```
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    /// If the `Value` is an Array, returns the associated vector. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": ["an", "array"], "b": { "an": "object" } });
    ///
    /// // The length of `["an", "array"]` is 2 elements.
    /// assert_eq!(v["a"].as_array().unwrap().len(), 2);
    ///
    /// // The object `{"an": "object"}` is not an array.
    /// assert_eq!(v["b"].as_array(), None);
    /// ```
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::Array(ref array) => Some(&*array),
            _ => None,
        }
    }

    /// If the `Value` is an Array, returns the associated mutable vector.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut v = json!({ "a": ["an", "array"] });
    ///
    /// v["a"].as_array_mut().unwrap().clear();
    /// assert_eq!(v, json!({ "a": [] }));
    /// ```
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match *self {
            Value::Array(ref mut list) => Some(list),
            _ => None,
        }
    }

    /// Returns true if the `Value` is a String. Returns false otherwise.
    ///
    /// For any Value on which `is_string` returns true, `as_str` is guaranteed
    /// to return the string slice.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": "some string", "b": false });
    ///
    /// assert!(v["a"].is_string());
    ///
    /// // The boolean `false` is not a string.
    /// assert!(!v["b"].is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    /// If the `Value` is a String, returns the associated str. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": "some string", "b": false });
    ///
    /// assert_eq!(v["a"].as_str(), Some("some string"));
    ///
    /// // The boolean `false` is not a string.
    /// assert_eq!(v["b"].as_str(), None);
    ///
    /// // JSON values are printed in JSON representation, so strings are in quotes.
    /// //
    /// //    The value is: "some string"
    /// println!("The value is: {}", v["a"]);
    ///
    /// // Rust strings are printed without quotes.
    /// //
    /// //    The value is: some string
    /// println!("The value is: {}", v["a"].as_str().unwrap());
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref s) => Some(s),
            _ => None,
        }
    }

    /// Returns true if the `Value` is a Number. Returns false otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 1, "b": "2" });
    ///
    /// assert!(v["a"].is_number());
    ///
    /// // The string `"2"` is a string, not a number.
    /// assert!(!v["b"].is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        match *self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    /// Returns true if the `Value` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any Value on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let big = i64::max_value() as u64 + 10;
    /// let v = json!({ "a": 64, "b": big, "c": 256.0 });
    ///
    /// assert!(v["a"].is_i64());
    ///
    /// // Greater than i64::MAX.
    /// assert!(!v["b"].is_i64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_i64());
    /// ```
    pub fn is_i64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_i64(),
            _ => false,
        }
    }

    /// Returns true if the `Value` is an integer between zero and `u64::MAX`.
    ///
    /// For any Value on which `is_u64` returns true, `as_u64` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert!(v["a"].is_u64());
    ///
    /// // Negative integer.
    /// assert!(!v["b"].is_u64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_u64());
    /// ```
    pub fn is_u64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_u64(),
            _ => false,
        }
    }

    /// Returns true if the `Value` is a number that can be represented by f64.
    ///
    /// For any Value on which `is_f64` returns true, `as_f64` is guaranteed to
    /// return the floating point value.
    ///
    /// Currently this function returns true if and only if both `is_i64` and
    /// `is_u64` return false but this is not a guarantee in the future.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 256.0, "b": 64, "c": -64 });
    ///
    /// assert!(v["a"].is_f64());
    ///
    /// // Integers.
    /// assert!(!v["b"].is_f64());
    /// assert!(!v["c"].is_f64());
    /// ```
    pub fn is_f64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_f64(),
            _ => false,
        }
    }

    /// If the `Value` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let big = i64::max_value() as u64 + 10;
    /// let v = json!({ "a": 64, "b": big, "c": 256.0 });
    ///
    /// assert_eq!(v["a"].as_i64(), Some(64));
    /// assert_eq!(v["b"].as_i64(), None);
    /// assert_eq!(v["c"].as_i64(), None);
    /// ```
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::Number(ref n) => n.as_i64(),
            _ => None,
        }
    }

    /// If the `Value` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert_eq!(v["a"].as_u64(), Some(64));
    /// assert_eq!(v["b"].as_u64(), None);
    /// assert_eq!(v["c"].as_u64(), None);
    /// ```
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::Number(ref n) => n.as_u64(),
            _ => None,
        }
    }

    /// If the `Value` is a number, represent it as f64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 256.0, "b": 64, "c": -64 });
    ///
    /// assert_eq!(v["a"].as_f64(), Some(256.0));
    /// assert_eq!(v["b"].as_f64(), Some(64.0));
    /// assert_eq!(v["c"].as_f64(), Some(-64.0));
    /// ```
    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::Number(ref n) => n.as_f64(),
            _ => None,
        }
    }

    /// Returns true if the `Value` is a Boolean. Returns false otherwise.
    ///
    /// For any Value on which `is_boolean` returns true, `as_bool` is
    /// guaranteed to return the boolean value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": false, "b": "false" });
    ///
    /// assert!(v["a"].is_boolean());
    ///
    /// // The string `"false"` is a string, not a boolean.
    /// assert!(!v["b"].is_boolean());
    /// ```
    pub fn is_boolean(&self) -> bool {
        self.as_bool().is_some()
    }

    /// If the `Value` is a Boolean, returns the associated bool. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": false, "b": "false" });
    ///
    /// assert_eq!(v["a"].as_bool(), Some(false));
    ///
    /// // The string `"false"` is a string, not a boolean.
    /// assert_eq!(v["b"].as_bool(), None);
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }

    /// Returns true if the `Value` is a Null. Returns false otherwise.
    ///
    /// For any Value on which `is_null` returns true, `as_null` is guaranteed
    /// to return `Some(())`.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": null, "b": false });
    ///
    /// assert!(v["a"].is_null());
    ///
    /// // The boolean `false` is not null.
    /// assert!(!v["b"].is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    /// If the `Value` is a Null, returns (). Returns None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": null, "b": false });
    ///
    /// assert_eq!(v["a"].as_null(), Some(()));
    ///
    /// // The boolean `false` is not null.
    /// assert_eq!(v["b"].as_null(), None);
    /// ```
    pub fn as_null(&self) -> Option<()> {
        match *self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    /// Takes the value out of the `Value`, leaving a `Null` in its place.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut v = json!({ "x": "y" });
    /// assert_eq!(v["x"].take(), json!("y"));
    /// assert_eq!(v, json!({ "x": null }));
    /// ```
    pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::Null
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Number {
    n: N,
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Self { n: N::Int(n) }
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Self::Number(Number::from(n))
    }
}

impl From<i64> for Number {
    fn from(n: i64) -> Self {
        Self { n: N::Long(n) }
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Self::Number(n.into())
    }
}

impl From<f32> for Number {
    fn from(n: f32) -> Self {
        Self { n: N::Float(n) }
    }
}

impl From<f32> for Value {
    fn from(n: f32) -> Self {
        Self::Number(n.into())
    }
}

impl From<f64> for Number {
    fn from(n: f64) -> Self {
        Self { n: N::Double(n) }
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Self::Number(n.into())
    }
}

#[derive(Copy, Clone, PartialEq)]
enum N {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}

// Implementing Eq is fine since any float values are always finite.
impl Eq for N {}

impl Number {
    /// Returns true if the `Number` is an integer between `i32::MIN` and
    /// `i32::MAX`.
    ///
    /// For any Number on which `is_i32` returns true, `as_i32` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let big = i32::max_value() as u32 + 10;
    /// let v = json!({ "a": 64, "b": big, "c": 256.0 });
    ///
    /// assert!(v["a"].is_i32());
    ///
    /// // Greater than i32::MAX.
    /// assert!(!v["b"].is_i32());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_i32());
    /// ```
    #[inline]
    pub fn is_i32(&self) -> bool {
        match self.n {
            N::Long(v) => i32::min_value() as i64 <= v && v <= i32::max_value() as i64,
            N::Int(_) => true,
            N::Float(_) | N::Double(_) => false,
        }
    }

    /// Returns true if the `Number` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any Number on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let big = i64::max_value() as u64 + 10;
    /// let v = json!({ "a": 64, "b": big, "c": 256.0 });
    ///
    /// assert!(v["a"].is_i64());
    ///
    /// // Greater than i64::MAX.
    /// assert!(!v["b"].is_i64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_i64());
    /// ```
    #[inline]
    pub fn is_i64(&self) -> bool {
        match self.n {
            N::Long(_) | N::Int(_) => true,
            N::Float(_) | N::Double(_) => false,
        }
    }

    /// Returns true if the `Number` is an integer between zero and `u32::MAX`.
    ///
    /// For any Number on which `is_u32` returns true, `as_u32` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert!(v["a"].is_u32());
    ///
    /// // Negative integer.
    /// assert!(!v["b"].is_u32());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_u32());
    /// ```
    #[inline]
    pub fn is_u32(&self) -> bool {
        match self.n {
            N::Long(n) => n >= 0 && n <= std::u32::MAX as i64,
            N::Int(n) => n >= 0,
            N::Float(_) | N::Double(_) => false,
        }
    }

    /// Returns true if the `Number` is an integer between zero and `u64::MAX`.
    ///
    /// For any Number on which `is_u64` returns true, `as_u64` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert!(v["a"].is_u64());
    ///
    /// // Negative integer.
    /// assert!(!v["b"].is_u64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_u64());
    /// ```
    #[inline]
    pub fn is_u64(&self) -> bool {
        match self.n {
            N::Int(n) => n >= 0,
            N::Long(n) => n >= 0,
            N::Float(_) | N::Double(_) => false,
        }
    }

    /// Returns true if the `Number` can be represented by f32.
    ///
    /// For any Number on which `is_f32` returns true, `as_f32` is guaranteed to
    /// return the floating point value.
    ///
    /// Currently this function returns true if and only if both `is_i32` and
    /// `is_u32` return false but this is not a guarantee in the future.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 256.0, "b": 64, "c": -64 });
    ///
    /// assert!(v["a"].is_f32());
    ///
    /// // Integers.
    /// assert!(!v["b"].is_f32());
    /// assert!(!v["c"].is_f32());
    /// ```
    #[inline]
    pub fn is_f32(&self) -> bool {
        match self.n {
            N::Float(_) => true,
            N::Int(_) | N::Long(_) | N::Double(_) => false,
        }
    }

    /// Returns true if the `Number` can be represented by f64.
    ///
    /// For any Number on which `is_f64` returns true, `as_f64` is guaranteed to
    /// return the floating point value.
    ///
    /// Currently this function returns true if and only if both `is_i64` and
    /// `is_u64` return false but this is not a guarantee in the future.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 256.0, "b": 64, "c": -64 });
    ///
    /// assert!(v["a"].is_f64());
    ///
    /// // Integers.
    /// assert!(!v["b"].is_f64());
    /// assert!(!v["c"].is_f64());
    /// ```
    #[inline]
    pub fn is_f64(&self) -> bool {
        match self.n {
            N::Double(_) => true,
            N::Int(_) | N::Long(_) | N::Float(_) => false,
        }
    }

    /// If the `Number` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let big = i64::max_value() as u64 + 10;
    /// let v = json!({ "a": 64, "b": big, "c": 256.0 });
    ///
    /// assert_eq!(v["a"].as_i64(), Some(64));
    /// assert_eq!(v["b"].as_i64(), None);
    /// assert_eq!(v["c"].as_i64(), None);
    /// ```
    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            N::Int(n) => Some(n as i64),
            N::Long(n) => Some(n),
            N::Float(_) | N::Double(_) => None,
        }
    }

    #[inline]
    pub fn as_i32(&self) -> Option<i32> {
        match self.n {
            N::Int(n) => Some(n),
            N::Long(n) => {
                (n >= i32::min_value() as i64 && n <= i32::max_value() as i64).then(|| n as i32)
            }
            N::Float(_) | N::Double(_) => None,
        }
    }

    /// If the `Number` is an integer, represent it as u32 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert_eq!(v["a"].as_u32(), Some(64));
    /// assert_eq!(v["b"].as_u32(), None;
    /// assert_eq!(v["c"].as_u32(), None);
    /// ```
    #[inline]
    pub fn as_u32(&self) -> Option<u32> {
        match self.n {
            N::Int(n) => (n > 0).then(|| n as u32),
            N::Long(n) => (n >= 0 && n <= u32::max_value() as i64).then(|| n as u32),
            N::Double(_) | N::Float(_) => None,
        }
    }

    /// If the `Number` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert_eq!(v["a"].as_u64(), Some(64));
    /// assert_eq!(v["b"].as_u64(), None;
    /// assert_eq!(v["c"].as_u64(), None);
    /// ```
    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            N::Int(n) => (n >= 0).then(|| n as u64),
            N::Long(n) => (n >= 0).then(|| n as u64),
            N::Double(_) | N::Float(_) => None,
        }
    }

    /// Represents the number as f64 if possible. Returns None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 256.0, "b": 64, "c": -64 });
    ///
    /// assert_eq!(v["a"].as_f64(), Some(256.0));
    /// assert_eq!(v["b"].as_f64(), Some(64.0));
    /// assert_eq!(v["c"].as_f64(), Some(-64.0));
    /// ```
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            N::Int(n) => Some(n as f64),
            N::Long(n) => Some(n as f64),
            N::Float(n) => Some(n as f64),
            N::Double(n) => Some(n),
        }
    }

    #[inline]
    pub fn as_f32(&self) -> Option<f32> {
        match self.n {
            N::Int(_) => None,
            N::Long(_) => None,
            N::Float(n) => Some(n as f32),
            N::Double(_) => None,
        }
    }

    /// Converts a finite `f64` to a `Number`. Infinite or NaN values are not JSON
    /// numbers.
    ///
    /// ```
    /// # use std::f64;
    /// #
    /// # use serde_json::Number;
    /// #
    /// assert!(Number::from_f64(256.0).is_some());
    ///
    /// assert!(Number::from_f64(f64::NAN).is_none());
    /// ```
    #[inline]
    pub fn from_f64(d: f64) -> Option<Number> {
        d.is_finite().then(|| Number { n: N::Double(d) })
    }
}

impl fmt::Display for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.n {
            N::Int(i) => fmt::Display::fmt(&i, formatter),
            N::Long(l) => fmt::Display::fmt(&l, formatter),
            N::Float(f) => fmt::Display::fmt(&f, formatter),
            N::Double(d) => fmt::Display::fmt(&d, formatter),
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = formatter.debug_tuple("Number");
        match self.n {
            N::Int(i) => {
                debug.field(&i);
            }
            N::Long(l) => {
                debug.field(&l);
            }
            N::Float(f) => {
                debug.field(&f);
            }
            N::Double(d) => {
                debug.field(&d);
            }
        }
        debug.finish()
    }
}
