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

use gazetta::prelude::*;
use gazetta::{ Site, Page };
use horrorshow::prelude::*;

use super::meta::{SourceMeta, EntryMeta};

pub struct MyGazetta;

template! {
    RenderDate(date: &::gazetta::model::Date) {
        time(datetime=format_args!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day())) {
            : format_args!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day())
        }
    }
}

impl MyGazetta {
    fn render_page_inner(&self, _site: &Site<Self>, page: &Page<Self>, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            header(id="page-header", class="title") {
                h1 : &page.title;
                : page.date.map(RenderDate::new);
            }
            @ if let Some(ref person) = page.author {
                span(id="page-author") {
                    @ if let Some(ref email) = person.email {
                        a(href=format_args!("mailto:{}", email)) : &person.name;
                    } else {
                        : &person.name
                    }
                }
            }
            @ if let Some(ref about) = page.about {
                div(id="about") {
                    @ if let Some(ref photo) = about.photo {
                        div(id="about-photo") {
                            img(src=photo, alt="Photo");
                        }
                    }
                    div(id="about-name") {
                        div(id="about-realname") : &about.name;
                        @ if !about.nicknames.is_empty() {
                            ul(id="about-nicks") {
                                @ for nick in &about.nicknames {
                                    li: nick
                                }
                            }
                        }
                    }
                    table(id="about-extra") {
                        @ if let Some(ref email) = about.email {
                            tr {
                                th : "Email";
                                td {
                                    a(href=format_args!("mailto:{}", email)): &about.email;
                                }
                            }
                        }
                        @ if !about.also.is_empty() {
                            tr {
                                th : "Also";
                                td {
                                    ul(id="about-also") {
                                        @ for link in &about.also {
                                            li {
                                                a(href=&link.url, rel="nofollow me"): &link.text;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        @ if let Some(ref key) = about.key {
                            tr {
                                th : "PGP Key";
                                td {
                                    a(href=&key.url): &key.fingerprint;
                                }
                            }
                        }
                    }
                }
            }
            @ if !page.content.data.trim().is_empty() {
                div(id="page-content", class="content") : page;
            }
            @ if let Some(ref idx) = page.index {
                div(id="page-index") {
                    @ for entry in idx.entries.iter() {
                        article {
                            header(class="title") {
                                h1 {
                                    a(href=&entry.href, rel="canonical") : &entry.title;
                                }
                                : entry.date.map(RenderDate::new);
                            }
                            div(class="content") : entry;
                        }
                    }
                }
                @ if let Some(ref paginate) = idx.paginate {
                    footer {
                        nav(id="page-pagination") {
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
        }
    }
}

impl Gazetta for MyGazetta {
    type SiteMeta = SourceMeta;
    type PageMeta = EntryMeta;

    fn render_page(&self, site: &Site<Self>, page: &Page<Self>, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            : raw!("<!DOCTYPE html>");
            html(lang="en") {
                head {
                    title : &page.title;
                    meta(name="viewport", content="width=device-width, initial-scale=1.0");
                    @ if let Some(ref person) = page.author {
                        meta(name="author", content=&person.name);
                    } else {
                        meta(name="author", content=&site.author.name);
                    }
                    : site;
                }
                body {
                    header(id="site-header") {
                        h1 {
                            a(href="/") : &site.title
                        }

                        @ if !site.nav.is_empty() {
                            nav(id="site-nav") {
                                ul {
                                    @ for link in &site.nav {
                                        li(class? = if page.href.starts_with(&link.url) {
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
                            section {
                                |tmpl| self.render_page_inner(site, page, tmpl);
                            }
                        } else {
                            article {
                                |tmpl| self.render_page_inner(site, page, tmpl);
                            }
                        }
                    }
                    footer(id="site-footer") {
                        p {
                            : raw!("&copy; ");
                            span(id="site-author") : &site.author.name;
                        }
                    }
                }
            }
        };
    }
}
