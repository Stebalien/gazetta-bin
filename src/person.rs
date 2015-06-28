use std::fmt;
use link::Link;

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub email: Option<String>,
    pub photo: Option<String>,
    pub key: Option<Key>,
    pub nicknames: Vec<String>,
    pub also: Vec<Link>,
}

#[derive(Debug, Clone)]
pub struct Key {
    pub url: String,
    pub fingerprint: String,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.name));
        if let Some(ref email) = self.email {
            try!(write!(f, " <{}>", email));
        }
        Ok(())
    }
}
