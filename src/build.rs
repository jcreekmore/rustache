pub use std::collections::{HashSet, HashMap};

use super::{Data, Static, Bool, Vector, Map};

pub struct Builder<'a> {
    data: HashMap<String, Data<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Builder<'a> {
        Builder {
            data: HashMap::new()
        }
    }

    pub fn insert_static<K: StrAllocating, V: StrAllocating>(self, key: K, value: V) -> Builder<'a> {
        let Builder { mut data } = self;
        data.insert(key.into_string(), Static(value.into_string()));
        Builder { data: data }
    }

    pub fn insert_bool<K: StrAllocating>(self, key: K, value: bool) -> Builder<'a> {
        let Builder { mut data } = self;
        data.insert(key.into_string(), Bool(value));
        Builder { data: data }
    }

    pub fn create_data_map<'a>(tags: HashSet<String>, data: HashMap<&'a str, &'a str>) -> HashMap<String, String> {
        let mut value_map = HashMap::new();

        for tag in tags.into_iter() {
            let datum = data.find_equiv(&tag.as_slice())
                .unwrap_or(&"")
                .to_string();
            value_map.insert(tag, datum);
        }

        value_map
    }
}
