use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}
/*
 * example query_str: a=1&b=2&c&d=&e===&d=7&d=abc
 */
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(query_str: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in query_str.split('&') {

            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key).and_modify(|existing: &mut Value| match existing {
                Value::Single(prev_val) => {
                    *existing = Value::Multiple(vec![prev_val, val])
                },
                Value::Multiple(vec) => {
                    vec.push(val);
                }
            }).or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}
