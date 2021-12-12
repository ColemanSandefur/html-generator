use crate::attributes::Attributes;
use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone, HTMLRendering)]
pub struct HTMLRow {
    #[rendered_iter("tr")]
    element: Vec<HTMLElement>,

    #[attributes]
    attributes: Attributes,
}

impl HTMLRow {
    pub fn new() -> Self {
        Self {
            element: Vec::new(),
            attributes: Attributes::new(),
        }
    }

    pub fn with_element<T: Into<HTMLElement>>(element: T) -> Self {
        Self {
            element: vec![element.into()],
            attributes: Attributes::new(),
        }
    }

    pub fn with_elements(elements: Vec<HTMLElement>) -> Self {
        Self {
            element: elements,
            attributes: Attributes::new(),
        }
    }

    pub fn get_attributes(&self) -> &Attributes {
        &self.attributes
    }
    pub fn get_mut_attributes(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}

impl From<HTMLRow> for HTMLElement {
    fn from(row: HTMLRow) -> Self {
        HTMLElement::Row(row)
    }
}

impl From<Vec<HTMLElement>> for HTMLRow {
    fn from(row: Vec<HTMLElement>) -> Self {
        Self {
            element: row,
            attributes: Attributes::new(),
        }
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
