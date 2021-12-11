use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone, HTMLRendering)]
pub struct HTMLTableData {
    #[rendered_iter("td")]
    element: Vec<HTMLElement>,
}

impl HTMLTableData {
    pub fn new() -> Self {
        Self {
            element: Vec::new(),
        }
    }

    pub fn with_elements(element: Vec<HTMLElement>) -> Self {
        Self { element }
    }
}

impl HTMLManipulation<HTMLElement> for HTMLTableData {
    fn get_elements(&self) -> &Vec<HTMLElement> {
        &self.element
    }

    fn get_mut_elements(&mut self) -> &mut Vec<HTMLElement> {
        &mut self.element
    }
}

impl From<HTMLTableData> for HTMLElement {
    fn from(t: HTMLTableData) -> Self {
        HTMLElement::TableData(t)
    }
}

impl From<HTMLElement> for HTMLTableData {
    fn from(e: HTMLElement) -> Self {
        Self { element: vec![e] }
    }
}
