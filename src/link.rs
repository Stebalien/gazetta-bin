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

use horrorshow::prelude::*;

#[derive(Clone, Hash, Debug)]
pub struct Link {
    pub text: String,
    pub url: String,
    pub title: Option<String>,
}


impl RenderOnce for Link {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

impl RenderMut for Link {
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

impl Render for Link {
    fn render(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            a(href=&self.url, title=self.title.as_ref()) : &self.text
        };
    }
}


