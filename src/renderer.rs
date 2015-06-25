use gazetta::prelude::*;
use gazetta::{ Site, Page };
use horrorshow::prelude::*;

use super::meta::{SourceMeta, EntryMeta};

pub struct MyGazetta;

template! {
    RenderDate(date: &::gazetta::model::Date) {
        time(itemprop="dateCreated", datetime=format_args!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day())) {
            : format_args!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day())
        }
    }
}

impl MyGazetta {
    fn render_about(&self, site: &Site<Self>, page: &Page<Self>, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            div(id="about", itemscope, itemtype="http://schema.org/Person") {
                div(id="about-header") {
                    div(id="about-logo") {
                        img(itemprop="image", src="/static/img/logo.png", alt="Logo");
                    }
                    div(id="about-name") {
                        div(id="about-realname", itemprop="name") : &site.author.name;
                        @ if !site.author.nicknames.is_empty() {
                            ul(id="about-nicks") {
                                @ for nick in &site.author.nicknames {
                                    li(itemprop="additionalName"): nick
                                }
                            }
                        }
                    }
                    table(id="about-extra") {
                        @ if let Some(ref email) = site.author.email {
                            tr {
                                th : "Email:";
                                td {
                                    a(itemprop="email",
                                      href=format_args!("mailto:{}", email)
                                      ): &site.author.email;
                                }
                            }
                        }
                        @ if let Some(ref key) = site.author.key {
                            tr {
                                th: "PGP Key:";
                                td {
                                    a(href=&key.url): &key.fingerprint;
                                }
                            }
                        }
                    }
                }
                div(itemprop="text", class="content") : page;
            }
        };
    }

    fn render_default(&self, _site: &Site<Self>, page: &Page<Self>, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            div(class="content", itemprop="text") : page;
        };
    }

    fn render_content(&self, site: &Site<Self>, page: &Page<Self>, tmpl: &mut TemplateBuffer) {
        match &*page.layout {
            "default" => self.render_default(site, page, tmpl),
            "about" => self.render_about(site, page, tmpl),
            unknown => tmpl.record_error(format!("unknown layout '{}'", unknown)),
        }
    }

    fn render_page_inner(&self, site: &Site<Self>, page: &Page<Self>, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            meta(itemprop="author",
                 itemscope,
                 itemtype="http://schema.org/Person",
                 itemref="site-author");
            header(id="page-header", class="title") {
                h1(itemprop="headline") : &page.title;
                : page.date.map(RenderDate::new);
            }
            div(id="page-content") {
                |tmpl| self.render_content(site, page, tmpl);
            }
            @ if let Some(ref idx) = page.index {
                div(id="page-index") {
                    @ for entry in idx.entries.iter() {
                        article(itemscope, itemtype="http://schema.org/Article", class="index-item") {
                            meta(itemprop="author",
                                 itemscope,
                                 itemtype="http://schema.org/Person",
                                 itemref="site-author");
                            header(class="title") {
                                h1(itemprop="headline") {
                                    a(href=&entry.href, itemprop="url sameAs") : &entry.title;
                                }
                                : entry.date.map(RenderDate::new);
                            }
                            |tmpl| self.render_content(site, entry, tmpl);
                        }
                    }
                }
                @ if let Some(ref paginate) = idx.paginate {
                    footer {
                        nav(class="pagination") {
                            div {
                                @ if paginate.current == 0 {
                                    span(class="prev disabled") : raw!("&larr; Previous");
                                } else {
                                    a(href=paginate.pages[paginate.current-1], class="prev", title="previous") : raw!("&larr; Previous");
                                }

                                span : format_args!("{} of {}", paginate.current + 1, paginate.pages.len());
                                @ if paginate.current + 1 == paginate.pages.len() {
                                    span(class="next disabled") : raw!("Next &rarr;");
                                } else {
                                    a(href=paginate.pages[paginate.current+1], class="next", title="next") : raw!("Next &rarr;");
                                }
                            }
                        }
                    }
                }
            }
        };
    }
}

impl Gazetta for MyGazetta {
    type SiteMeta = SourceMeta;
    type PageMeta = EntryMeta;

    fn render_page(&self, site: &Site<Self>, page: &Page<Self>, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            : raw!("<!DOCTYPE html>");
            html(lang="en", itemtype="http://schema.org/WebSite", itemscope) {
                head {
                    meta(charset="utf-8");
                    title : &page.title;
                    meta(name="viewport", content="width=device-width, initial-scale=1.0");
                    meta(name="author", content=&site.author.name);
                    link(rel="stylesheet", href="/static/stylesheets/reset.css");
                    link(rel="stylesheet", href="/static/stylesheets/highlight.css");
                    link(rel="stylesheet", href="/static/stylesheets/main.css");
                    link(rel="shortcut icon", href="/static/img/icon.png");
                    script(src="/static/javascript/highlight.js") {}
                    script : raw!("hljs.configure({languages: []}); hljs.initHighlightingOnLoad();");
                }
                body {
                    header(id="site-header") {
                        h1(itemprop="headline name") {
                            a(class="brand", href="/") : &site.title
                        }

                        @ if !site.nav.is_empty() {
                            nav(id="site-nav") {
                                ul {
                                    @ for link in &site.nav {
                                        li(class? = if page.href.starts_with(&link.href) {
                                            Some("active")
                                        } else {
                                            None
                                        }) : link
                                    }
                                }
                            }
                        }
                    }
                    main(id="site-content") {
                        @ if page.content.data.trim().is_empty() {
                            section(itemscope, itemtype="http://schema.org/WebPage") {
                                |tmpl| self.render_page_inner(site, page, tmpl);
                            }
                        } else {
                            article(itemscope, itemtype="http://schema.org/Article") {
                                |tmpl| self.render_page_inner(site, page, tmpl);
                            }
                        }
                    }
                    footer(id="site-footer", class="copyright") {
                        p {
                            : raw!("&copy; ");
                            span(itemprop="author copyrightHolder",
                                 itemscope,
                                 itemtype="http://schema.org/Person") {
                                span(id="site-author") {
                                    span(itemprop="name", class="fn") : &site.author.name;
                                    @ if let Some(ref e) = site.author.email {
                                        meta(class="email", itemprop="email", content=e)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
    }
}
