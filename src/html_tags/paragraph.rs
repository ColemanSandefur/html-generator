use crate::html_tags::HTMLElement;
use crate::html_tags::HTMLRendering;

#[derive(Clone)]
pub struct HTMLParagraph {
    text: String,
}

impl HTMLParagraph {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl HTMLRendering for HTMLParagraph {
    fn render(&self) -> String {
        format!("<p>{}</p>", &self.text)
    }
}

impl From<HTMLParagraph> for HTMLElement {
    fn from(p: HTMLParagraph) -> Self {
        HTMLElement::Paragraph(p)
    }
}
