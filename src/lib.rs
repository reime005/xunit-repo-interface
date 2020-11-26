#[macro_use]
extern crate serde_derive;
pub use xunit_struct::errors::XunitError;
pub use xunit_struct::model::Xunit;
pub use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub content: Xunit,
    pub directory: String,
    pub filename: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enviroment {
    pub sk: Option<String>,
    pub key_value: Vec<KeyValue>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub sk: Option<String>,
    pub identiifier: Option<String>,
    pub human_name: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Run {
    pub sk: Option<String>,
    pub client_identifier: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Upload {
    pub project: Project,
    pub enviroment: Enviroment,
    pub run: Run,
    pub files: Vec<File>,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}