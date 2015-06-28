use gazetta::yaml::{Hash, Yaml};
use gazetta::Meta;
use link::Link;
use person::{Person, Key};

lazy_static! {
    static ref NAV: Yaml = Yaml::String(String::from("nav"));
    static ref NAME: Yaml = Yaml::String("name".into());
    static ref ABOUT: Yaml = Yaml::String(String::from("about"));
    static ref AUTHOR: Yaml = Yaml::String("author".into());
    static ref EMAIL: Yaml = Yaml::String("email".into());
    static ref KEY: Yaml = Yaml::String("key".into());
    static ref URL: Yaml = Yaml::String("url".into());
    static ref FINGERPRINT: Yaml = Yaml::String("fingerprint".into());
    static ref NICKNAMES: Yaml = Yaml::String("nicknames".into());
    static ref PHOTO: Yaml = Yaml::String("photo".into());
    static ref ALSO: Yaml = Yaml::String("also".into());
}

trait OptionExt {
    type Value;
    type Error;
    fn invert(self) -> Result<Option<Self::Value>, Self::Error>;
}
impl<V, E> OptionExt for Option<Result<V, E>> {
    type Value = V;
    type Error = E;
    fn invert(self) -> Result<Option<Self::Value>, Self::Error> {
        match self {
            Some(Ok(v)) => Ok(Some(v)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }
}

fn parse_links(links: Yaml) -> Result<Vec<Link>, &'static str> {
    Ok(match links {
        Yaml::Array(links) => try!(links.into_iter().map(|link| match link {
            Yaml::Hash(link) => {
                if link.len() != 1 {
                    return Err("links must have exactly one entry".into());
                }
                match link.into_iter().next().unwrap() {
                    (Yaml::String(k), Yaml::String(v)) => Ok(Link {
                        text: k,
                        url: v,
                        title: None
                    }),
                    _ => Err("links must be in the form `name: url`"),
                }
            },
            _ => return Err("links must be in the form `name: url`"),
        }).collect()),
        _ => return Err("lists of links need to be arrays"),
    })
}

fn parse_person(person: Yaml) -> Result<Person, &'static str> {
    Ok(match person {
        Yaml::Hash(mut person) => Person {
            name: match person.remove(&NAME) {
                Some(Yaml::String(name)) => name,
                None => return Err("missing name"),
                _ => return Err("name must be a string"),
            },
            photo: match person.remove(&PHOTO) {
                Some(Yaml::String(photo)) => Some(photo),
                None => None,
                _ => return Err("if specified, photo must be a string"),
            },
            email: match person.remove(&EMAIL) {
                Some(Yaml::String(email)) => Some(email),
                None => None,
                _ => return Err("if specified, email must be a string"),
            },
            nicknames: match person.remove(&NICKNAMES) {
                Some(Yaml::String(nick)) => vec![nick],
                Some(Yaml::Array(nicks)) => try!(nicks.into_iter().map(|nick| match nick {
                    Yaml::String(nick) => Ok(nick),
                    _ => Err("nicknames must be strings"),
                }).collect()),
                Some(..) => return Err("invalid nicknames value"),
                None => vec![],
            },
            also: try!(person.remove(&ALSO).map(parse_links).invert()).unwrap_or(Vec::new()),
            key: match person.remove(&KEY) {
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
        Yaml::String(name) => Person {
            name: name,
            email: None,
            key: None,
            also: Vec::new(),
            photo: None,
            nicknames: Vec::new(),
        },
        _ => return Err("invalid person"),
    })
}

pub struct SourceMeta {
    pub nav: Vec<Link>,
    pub author: Person,
}

impl Meta for SourceMeta {
    fn from_yaml(mut meta: Hash) -> Result<SourceMeta, &'static str> {
        Ok(SourceMeta {
            nav: try!(meta.remove(&NAV).map(parse_links).invert()).unwrap_or(Vec::new()),
            author: match try!(meta.remove(&AUTHOR).map(parse_person).invert()) {
                Some(person) => person,
                None => return Err("websites must have authors"),
            },
        })
    }
}

pub struct EntryMeta {
    pub author: Option<Person>,
    pub about: Option<Person>,
}

impl Meta for EntryMeta {
    fn from_yaml(mut meta: Hash) -> Result<EntryMeta, &'static str> {
        Ok(EntryMeta {
            author: try!(meta.remove(&AUTHOR).map(parse_person).invert()),
            about: try!(meta.remove(&ABOUT).map(parse_person).invert()),
        })
    }
}
