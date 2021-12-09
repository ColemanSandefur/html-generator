use crate::html_tags::HTMLElement;
use crate::prelude::*;

#[derive(Clone)]
pub struct HTMLTableData {
    element: Vec<HTMLElement>,
}

impl HTMLTableData {
    pub fn new() -> Self {
        Self {
            element: Vec::new(),
        }
    }
}

impl HTMLRendering for HTMLTableData {
    fn render(&self) -> String {
        let mut output = String::new();

        for item in &self.element {
            output.push_str(&item.render())
        }

        format!("<td>{}</td>", output)
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

impl<T: Into<HTMLElement>> From<T> for HTMLTableData {
    fn from(t: T) -> Self {
        Self {
            element: vec![t.into()],
        }
    }
}
