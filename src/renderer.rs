/*  Copyright (C) 2015 Steven Allen
 *
 *  This file is part of gazetta.
 *
 *  This program is free software: you can redistribute it and/or modify it under the terms of the
 *  GNU General Public License as published by the Free Software Foundation version 3 of the
 *  License.
 *
 *  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 *  without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See
 *  the GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License along with this program.  If
 *  not, see <http://www.gnu.org/licenses/>.
 */

use gazetta::prelude::*;
use gazetta::render;
use gazetta::{EntryMeta, Page, Site, SourceMeta};
use horrorshow::helper::doctype;
use horrorshow::html;
use horrorshow::prelude::*;

pub struct MyGazetta;

impl MyGazetta {
    fn render_page_inner(&self, page: &Page<Self>, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            header(id="page-header", class="title") {
                h1(class="header") : &page.title;
                : page.date.map(render::Date);
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
                div(id="page-content", class="content") : render::Content(page);
            }
            @ if let Some(ref idx) = page.index {
                div(id="page-index") {
                    @ for entry in idx.entries.iter() {
                        article {
                            header(class="title") {
                                h1(class="header") {
                                    a(href=&entry.href) : &entry.title;
                                }
                                : entry.date.map(render::Date);
                            }
                            @ if idx.compact {
                                @ if let Some(desc) = entry.description {
                                    div(class="content") : render::Markdown::new(desc, entry.href);
                                }
                            } else {
                                div(class="content") : render::Content(entry);
                            }
                        }
                    }
                }
                @ if let Some(ref paginate) = idx.paginate {
                    footer {
                        nav(id="page-pagination") {
                            div {
                                @ if paginate.current == 0 {
                                    span(class="prev disabled") : Raw("&larr; Previous");
                                } else {
                                    a(href=paginate.pages[paginate.current-1],
                                      class="prev",
                                      rel="prev",
                                      title="previous"
                                     ) : Raw("&larr; Previous");
                                }
                                span : format_args!(" {} of {} ", paginate.current + 1, paginate.pages.len());
                                @ if paginate.current + 1 == paginate.pages.len() {
                                    span(class="next disabled") : Raw("Next &rarr;");
                                } else {
                                    a(href=paginate.pages[paginate.current+1],
                                      class="next",
                                      rel="next",
                                      title="next"
                                     ) : Raw("Next &rarr;");
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
            : doctype::HTML;
            html(lang="en") {
                head {
                    link(rel="canonical", href=format_args!("{}{}{}", site.origin, site.prefix, page.href));
                    meta(charset="utf-8");
                    meta(name="viewport",
                         content="width=device-width, initial-scale=1.0");
                    @ if let Some(ref person) = page.author {
                        meta(name="author", content=&person.name);
                    } else {
                        meta(name="author", content=&site.author.name);
                    }
                    title : &page.title;

                    : render::Assets(site);
                }
                body {
                    header(id="site-header") {
                        a(class="header", href="") : &site.title;
                        : " ";
                        @ if !site.nav.is_empty() {
                            nav(id="site-nav") {
                                @ for link in &site.nav {
                                    // Otherwise, they run together on text
                                    // browsers
                                    : " ";
                                    a(href=&link.url, class? = if page.href.starts_with(&link.url) {
                                        Some("active")
                                    } else {
                                        None
                                    }) : &link.text;
                                }
                            }
                        }
                    }
                    main(id="site-content") {
                        @ if page.content.data.trim().is_empty() {
                            section {
                                |tmpl| self.render_page_inner(page, tmpl);
                            }
                        } else {
                            article {
                                |tmpl| self.render_page_inner(page, tmpl);
                            }
                        }
                    }
                    footer(id="site-footer") {
                        p {
                            : Raw("&copy; ");
                            span(id="site-author") : &site.author.name;
                        }
                    }
                }
            }
        };
    }
}
