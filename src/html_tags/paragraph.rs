use crate::attributes::Attributes;
use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone, HTMLRendering)]
pub struct HTMLParagraph {
    #[rendered_iter("p")]
    element: Vec<HTMLElement>,

    #[attributes]
    attributes: Attributes,
}

impl HTMLParagraph {
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

impl From<HTMLParagraph> for HTMLElement {
    fn from(p: HTMLParagraph) -> Self {
        HTMLElement::Paragraph(p)
    }
}

impl From<String> for HTMLParagraph {
    fn from(s: String) -> Self {
        HTMLParagraph::with_elements(vec![s.into()])
    }
}

impl From<&str> for HTMLParagraph {
    fn from(s: &str) -> Self {
        HTMLParagraph::with_elements(vec![s.into()])
    }
}

impl From<Vec<HTMLElement>> for HTMLParagraph {
    fn from(element: Vec<HTMLElement>) -> Self {
        Self {
            element,
            attributes: Attributes::new(),
        }
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
