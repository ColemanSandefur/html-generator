use crate::attributes::Attributes;
use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone, HTMLRendering)]
pub struct HTMLBody {
    #[rendered_iter("body")]
    element: Vec<HTMLElement>,

    #[attributes]
    attributes: Attributes,
}

impl HTMLBody {
    pub fn new() -> Self {
        Self {
            element: Vec::new(),
            attributes: Attributes::new(),
        }
    }

    pub fn set_content(&mut self, element: Vec<HTMLElement>) {
        self.element = element;
    }

    pub fn get_attributes(&self) -> &Attributes {
        &self.attributes
    }
    pub fn get_mut_attributes(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}

impl From<HTMLBody> for HTMLElement {
    fn from(b: HTMLBody) -> Self {
        HTMLElement::Body(b)
    }
}

impl From<Vec<HTMLElement>> for HTMLBody {
    fn from(element: Vec<HTMLElement>) -> Self {
        Self {
            element,
            attributes: Attributes::new(),
        }
    }
}

impl HTMLManipulation<HTMLElement> for HTMLBody {
    fn get_elements(&self) -> &Vec<HTMLElement> {
        &self.element
    }

    fn get_mut_elements(&mut self) -> &mut Vec<HTMLElement> {
        &mut self.element
    }
}
