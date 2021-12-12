use crate::attributes::Attributes;
use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone, HTMLRendering)]
pub struct HTMLTableHeader {
    #[rendered_iter("th")]
    element: Vec<HTMLElement>,

    #[attributes]
    attributes: Attributes,
}

impl HTMLTableHeader {
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
        Self {
            element: vec![t],
            attributes: Attributes::new(),
        }
    }
}
