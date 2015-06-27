use std::borrow::Cow;

use gazetta::yaml::{Hash, Yaml};
use gazetta::Meta;
use link::Link;
use person::{Person, Key};

lazy_static! {
    static ref NAV: Yaml = Yaml::String(String::from("nav"));
    static ref NAME: Yaml = Yaml::String("name".into());
    static ref LAYOUT: Yaml = Yaml::String(String::from("layout"));
    static ref AUTHOR: Yaml = Yaml::String("author".into());
    static ref EMAIL: Yaml = Yaml::String("email".into());
    static ref KEY: Yaml = Yaml::String("key".into());
    static ref URL: Yaml = Yaml::String("url".into());
    static ref FINGERPRINT: Yaml = Yaml::String("fingerprint".into());
    static ref NICKNAMES: Yaml = Yaml::String("nicknames".into());
    static ref PHOTO: Yaml = Yaml::String("photo".into());
}

pub struct SourceMeta {
    pub nav: Vec<Link>,
    pub author: Person,
}

impl Meta for SourceMeta {
    fn from_yaml(mut meta: Hash) -> Result<SourceMeta, &'static str> {
        Ok(SourceMeta {
            nav: match meta.remove(&NAV) {
                Some(Yaml::Array(nav_data)) => try!(nav_data.into_iter().map(|item| match item {
                    Yaml::Hash(item) => {
                        if item.len() != 1 {
                            return Err("nav mappings must have exactly one entry".into());
                        }
                        match item.into_iter().next().unwrap() {
                            (Yaml::String(k), Yaml::String(v)) => Ok(Link {
                                text: k,
                                href: v,
                                title: None
                            }),
                            _ => Err("nav items must be in the form `title: url`"),
                        }
                    },
                    _ => return Err("nav items must be in the form `title: url`"),
                }).collect()),
                Some(..) => return Err("You must provide a list of nav items"),
                None => Vec::new(),
            },
            author: match meta.remove(&AUTHOR) {
                Some(Yaml::Hash(mut author)) => Person {
                    name: match author.remove(&NAME) {
                        Some(Yaml::String(name)) => name,
                        None => return Err("missing author name"),
                        _ => return Err("author name must be a string"),
                    },
                    photo: match author.remove(&PHOTO) {
                        Some(Yaml::String(photo)) => Some(photo),
                        None => None,
                        _ => return Err("if specified, author photo must be a string"),
                    },
                    email: match author.remove(&EMAIL) {
                        Some(Yaml::String(email)) => Some(email),
                        None => None,
                        _ => return Err("if specified, author email must be a string"),
                    },
                    nicknames: match author.remove(&NICKNAMES) {
                        Some(Yaml::String(nick)) => vec![nick],
                        Some(Yaml::Array(nicks)) => try!(nicks.into_iter().map(|nick| match nick {
                            Yaml::String(nick) => Ok(nick),
                            _ => Err("nicknames must be strings"),
                        }).collect()),
                        Some(..) => return Err("invalid nicknames value"),
                        None => vec![],
                    },
                    key: match author.remove(&KEY) {
                        Some(Yaml::Hash(mut key)) => Some(Key {
                            url: match key.remove(&URL) {
                                Some(Yaml::String(url)) => url,
                                Some(..) => return Err("key url must be a string"),
                                None => return Err("key url missing"),
                            },
                            fingerprint: match key.remove(&FINGERPRINT) {
                                Some(Yaml::String(fprint)) => fprint,
                                Some(..) => return Err("key fingerprint must be a string"),
                                None => return Err("key fingerprint missing"),
                            },
                        }),
                        Some(..) => return Err("if specified, key must be a hash"),
                        None => None,
                    }
                },
                Some(Yaml::String(name)) => Person {
                    name: name,
                    email: None,
                    key: None,
                    photo: None,
                    nicknames: Vec::new(),
                },
                Some(..) => return Err("invalid author"),
                None => return Err("must specify author"),
            },
        })
    }
}

pub struct EntryMeta {
    pub layout: Cow<'static, str>,
}

impl Meta for EntryMeta {
    fn from_yaml(mut meta: Hash) -> Result<EntryMeta, &'static str> {
        Ok(EntryMeta {
            layout: match meta.remove(&LAYOUT) {
                Some(Yaml::String(layout)) => Cow::Owned(layout),
                Some(..) => return Err("layout must be a string"),
                None => Cow::Borrowed("default"),
            },
        })
    }
}
