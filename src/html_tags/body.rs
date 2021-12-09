use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone)]
pub struct HTMLBody {
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

impl HTMLRendering for HTMLBody {
    fn render(&self) -> String {
        let mut output = String::new();

        for el in &self.element {
            output.push_str(&el.render());
        }

        format!("<body>{}</body>", &output)
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
