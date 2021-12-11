use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone, HTMLRendering)]
pub struct HTMLTableHeader {
    #[rendered_iter("th")]
    element: Vec<HTMLElement>,
}

impl HTMLTableHeader {
    pub fn new(element: Vec<HTMLElement>) -> Self {
        Self { element }
    }
}

impl From<HTMLTableHeader> for HTMLElement {
    fn from(t: HTMLTableHeader) -> HTMLElement {
        HTMLElement::TableHeader(t)
    }
}

impl HTMLManipulation<HTMLElement> for HTMLTableHeader {
    fn get_elements(&self) -> &Vec<HTMLElement> {
        &self.element
    }

    fn get_mut_elements(&mut self) -> &mut Vec<HTMLElement> {
        &mut self.element
    }
}

impl From<HTMLElement> for HTMLTableHeader {
    fn from(t: HTMLElement) -> Self {
        Self { element: vec![t] }
    }
}
