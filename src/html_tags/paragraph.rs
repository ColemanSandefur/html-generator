use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone)]
pub struct HTMLParagraph {
    element: Vec<HTMLElement>,
}

impl HTMLParagraph {
    pub fn new(element: Vec<HTMLElement>) -> Self {
        Self { element }
    }
}

impl HTMLRendering for HTMLParagraph {
    fn render(&self) -> String {
        let mut output = String::new();

        for el in &self.element {
            output.push_str(&el.render());
        }

        format!("<p>{}</p>", &output)
    }
}

impl From<HTMLParagraph> for HTMLElement {
    fn from(p: HTMLParagraph) -> Self {
        HTMLElement::Paragraph(p)
    }
}

impl From<String> for HTMLParagraph {
    fn from(s: String) -> Self {
        HTMLParagraph::new(vec![s.into()])
    }
}

impl From<&str> for HTMLParagraph {
    fn from(s: &str) -> Self {
        HTMLParagraph::new(vec![s.into()])
    }
}

impl From<Vec<HTMLElement>> for HTMLParagraph {
    fn from(element: Vec<HTMLElement>) -> Self {
        Self { element }
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
