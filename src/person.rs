/* Copyright (2015) Stevem Allen
 *
 * This file is part of gazetta-bin.
 * 
 * gazetta-bin is free software: you can redistribute it and/or modify it under the terms of the
 * GNU Affero General Public License (version 3) as published by the Free Software Foundation.
 * 
 * gazetta-bin is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
 * the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License along with gazetta-bin.  If
 * not, see <http://www.gnu.org/licenses/>.
 */

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
