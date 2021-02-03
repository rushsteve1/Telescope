use crate::{
    templates::{page, Template},
};
use actix_web::{HttpResponse, HttpRequest};
use std::marker::PhantomData;
use crate::error::TelescopeError;
use futures::future::Ready;
use futures::future::ready;

pub mod index;
pub mod projects;
pub mod sponsors;

/// A piece of static content. This is just a reference to a
/// handlebars file and some metadata for rendering the page.
pub trait StaticPage {
    /// The path to the handlebars file.
    const TEMPLATE_NAME: &'static str;

    /// The title of this page.
    const PAGE_TITLE: &'static str;

    /// Make the static template that this refers to.
    fn template() -> Template {
        Template::new(Self::TEMPLATE_NAME)
    }

    /// Create a page containing the static content.
    fn page(req: &HttpRequest) -> Result<Template, TelescopeError> {
        page::of(req, Self::PAGE_TITLE, &Self::template())
    }

    /// Actix handler that can be used to generate responses. This just wraps
    /// the page in an immediately ready future.
    fn handle(req: HttpRequest) -> Ready<Result<Template, TelescopeError>> {
        ready(Self::page(&req))
    }
}
