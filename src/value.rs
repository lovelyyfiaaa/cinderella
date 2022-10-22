use std::collections::HashMap;

///
///
/// Since [Option] is not provided, you should use [DynamicValue::Null]~!
pub enum DynamicValue {
  String(String),
  StaticString(&'static str),
  Int(usize),
  Boolean(bool),
  Null,
  HashMap(HashMap<String, Self>),
}
