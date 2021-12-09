use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone)]
pub struct HTMLHeading {
    element: Vec<HTMLElement>,

    strength: u8,
}

impl HTMLHeading {
    pub fn new(strength: u8) -> Self {
        Self {
            element: Vec::new(),
            strength,
        }
    }

    pub fn new_with_content(strength: u8, element: Vec<HTMLElement>) -> Self {
        Self { element, strength }
    }

    pub fn set_content(&mut self, element: Vec<HTMLElement>) {
        self.element = element;
    }
}

impl HTMLRendering for HTMLHeading {
    fn render(&self) -> String {
        let mut output = String::new();

        for el in &self.element {
            output.push_str(&el.render());
        }

        format!("<h{}>{}</h{}>", self.strength, &output, self.strength)
    }
}

impl From<HTMLHeading> for HTMLElement {
    fn from(h: HTMLHeading) -> Self {
        HTMLElement::Heading(h)
    }
}

impl From<Vec<HTMLElement>> for HTMLHeading {
    fn from(element: Vec<HTMLElement>) -> Self {
        Self {
            element,
            strength: 1,
        }
    }
}

impl HTMLManipulation<HTMLElement> for HTMLHeading {
    fn get_elements(&self) -> &Vec<HTMLElement> {
        &self.element
    }

    fn get_mut_elements(&mut self) -> &mut Vec<HTMLElement> {
        &mut self.element
    }
}
