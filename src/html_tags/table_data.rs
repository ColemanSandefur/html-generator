use crate::attributes::Attributes;
use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone, HTMLRendering)]
pub struct HTMLTableData {
    #[rendered_iter("td")]
    element: Vec<HTMLElement>,

    #[attributes]
    attributes: Attributes,
}

impl HTMLTableData {
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
        Self {
            element: vec![e],
            attributes: Attributes::new(),
        }
    }
}
