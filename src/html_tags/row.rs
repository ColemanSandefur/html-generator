use crate::html_tags::HTMLElement;
use crate::{prelude::*, HTMLTableData};

#[derive(Clone)]
pub struct HTMLRow {
    element: Vec<HTMLTableData>,
}

impl HTMLRendering for HTMLRow {
    fn render(&self) -> String {
        let mut output = String::new();

        output.push_str("<tr>");

        for element in &self.element {
            output.push_str(&element.render());
        }

        output.push_str("</tr>");

        output
    }
}

impl HTMLRow {
    pub fn new() -> Self {
        Self {
            element: Vec::new(),
        }
    }
}

impl From<HTMLRow> for HTMLElement {
    fn from(row: HTMLRow) -> Self {
        HTMLElement::Row(row)
    }
}

impl From<Vec<HTMLTableData>> for HTMLRow {
    fn from(row: Vec<HTMLTableData>) -> Self {
        Self { element: row }
    }
}

impl HTMLManipulation<HTMLTableData> for HTMLRow {
    fn get_elements(&self) -> &Vec<HTMLTableData> {
        &self.element
    }

    fn get_mut_elements(&mut self) -> &mut Vec<HTMLTableData> {
        &mut self.element
    }
}
