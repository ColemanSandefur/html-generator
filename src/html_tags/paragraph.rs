use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone, HTMLRendering)]
pub struct HTMLParagraph {
    #[rendered_iter("p")]
    element: Vec<HTMLElement>,
}

impl HTMLParagraph {
    pub fn new(element: Vec<HTMLElement>) -> Self {
        Self { element }
    }
}

impl From<HTMLParagraph> for HTMLElement {
    fn from(p: HTMLParagraph) -> Self {
        HTMLElement::Paragraph(p)
    }
}

impl From<String> for HTMLParagraph {
    fn from(s: String) -> Self {
        HTMLParagraph::new(vec![s.into()])
    }
}

impl From<&str> for HTMLParagraph {
    fn from(s: &str) -> Self {
        HTMLParagraph::new(vec![s.into()])
    }
}

impl From<Vec<HTMLElement>> for HTMLParagraph {
    fn from(element: Vec<HTMLElement>) -> Self {
        Self { element }
    }
}

impl HTMLManipulation<HTMLElement> for HTMLParagraph {
    fn get_elements(&self) -> &Vec<HTMLElement> {
        &self.element
    }

    fn get_mut_elements(&mut self) -> &mut Vec<HTMLElement> {
        &mut self.element
    }
}
