use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        // create a new HashMap to store query params as key value pairs 
        let mut data = HashMap::new();
        // iterate over every param by splitting on the & characters
        // assign the queries to 'key' and the pararms to 'value'
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }
            // see if the key already exists in the hashmap
            data.entry(key)
            .and_modify(|existing: &mut Value| match existing { 
                Value::Single(prev) => {
                    *existing = Value::Multiple(vec![prev, val]);
                }
                Value::Multiple(vec) => {vec.push(val);}
             })
            .or_insert(Value::Single(val));
            println!("A new query has been processed: query = {}, param = {}", &key, &val);
        }
        

        QueryString { data }

    }
}