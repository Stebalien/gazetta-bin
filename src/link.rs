use horrorshow::prelude::*;

pub struct Link {
    pub text: String,
    pub href: String,
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
            a(href=&self.href, title=self.title.as_ref()) : &self.text
        };
    }
}


