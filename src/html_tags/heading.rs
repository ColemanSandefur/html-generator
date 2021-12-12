use crate::attributes::Attributes;
use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone)]
pub struct HTMLHeading {
    element: Vec<HTMLElement>,

    strength: u8,

    attributes: Attributes,
}

impl HTMLHeading {
    pub fn new(strength: u8) -> Self {
        Self {
            element: Vec::new(),
            strength,
            attributes: Attributes::new(),
        }
    }

    pub fn with_element<T: Into<HTMLElement>>(strength: u8, element: T) -> Self {
        Self {
            element: vec![element.into()],
            strength,
            attributes: Attributes::new(),
        }
    }

    pub fn with_elements(strength: u8, elements: Vec<HTMLElement>) -> Self {
        Self {
            element: elements,
            strength,
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

impl HTMLRendering for HTMLHeading {
    fn render(&self) -> String {
        let mut output = String::new();

        for el in &self.element {
            output.push_str(&el.render());
        }

        format!(
            "<h{} {}>{}</h{}>",
            self.strength,
            self.attributes.render(),
            &output,
            self.strength
        )
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
            attributes: Attributes::new(),
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
