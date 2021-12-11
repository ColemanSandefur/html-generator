use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone, HTMLRendering)]
pub struct HTMLBody {
    #[rendered_iter("body")]
    element: Vec<HTMLElement>,
}

impl HTMLBody {
    pub fn new() -> Self {
        Self {
            element: Vec::new(),
        }
    }

    pub fn set_content(&mut self, element: Vec<HTMLElement>) {
        self.element = element;
    }
}

impl From<HTMLBody> for HTMLElement {
    fn from(b: HTMLBody) -> Self {
        HTMLElement::Body(b)
    }
}

impl From<Vec<HTMLElement>> for HTMLBody {
    fn from(element: Vec<HTMLElement>) -> Self {
        Self { element }
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
