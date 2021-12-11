use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone, HTMLRendering)]
pub struct HTMLRow {
    #[rendered_iter("tr")]
    element: Vec<HTMLElement>,
}

impl HTMLRow {
    pub fn new() -> Self {
        Self {
            element: Vec::new(),
        }
    }
}

impl From<HTMLRow> for HTMLElement {
    fn from(row: HTMLRow) -> Self {
        HTMLElement::Row(row)
    }
}

impl From<Vec<HTMLElement>> for HTMLRow {
    fn from(row: Vec<HTMLElement>) -> Self {
        Self { element: row }
    }
}

impl HTMLManipulation<HTMLElement> for HTMLRow {
    fn get_elements(&self) -> &Vec<HTMLElement> {
        &self.element
    }

    fn get_mut_elements(&mut self) -> &mut Vec<HTMLElement> {
        &mut self.element
    }
}
