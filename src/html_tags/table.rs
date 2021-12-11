use crate::attributes::Attributes;
use crate::html_tags::HTMLElement;
use crate::html_tags::HTMLRow;
use crate::prelude::*;

#[derive(Clone)]
pub struct HTMLTable {
    head: Vec<HTMLRow>,
    rows: Vec<HTMLRow>,

    attributes: Attributes,
}

impl HTMLRendering for HTMLTable {
    fn render(&self) -> String {
        let mut output = String::new();

        output.push_str("<table><tbody>");

        for row in &self.rows {
            output.push_str(&row.render());
        }

        output.push_str("</tbody></table>");

        output
    }
}

impl HTMLTable {
    pub fn new() -> Self {
        Self {
            head: Vec::new(),
            rows: Vec::new(),
            attributes: Attributes::new(),
        }
    }

    pub fn with_rows(rows: Vec<HTMLRow>) -> Self {
        Self {
            head: Vec::new(),
            rows,
            attributes: Attributes::new(),
        }
    }

    pub fn push_row(&mut self, row: HTMLRow) {
        self.rows.push(row);
    }

    pub fn pop_row(&mut self) -> Option<HTMLRow> {
        self.rows.pop()
    }

    pub fn remove_row(&mut self, index: usize) -> HTMLRow {
        self.rows.remove(index)
    }

    pub fn insert_row(&mut self, row: HTMLRow, index: usize) {
        self.rows.insert(index, row);
    }

    pub fn set_row(&mut self, row: HTMLRow, index: usize) -> HTMLRow {
        let old_row = self.rows.remove(index);
        self.rows.insert(index, row);

        old_row
    }

    pub fn get_head(&mut self, row: Vec<HTMLRow>) {
        self.head = row;
    }
}

impl From<HTMLTable> for HTMLElement {
    fn from(t: HTMLTable) -> Self {
        HTMLElement::Table(t)
    }
}

impl HTMLManipulation<HTMLRow> for HTMLTable {
    fn get_elements(&self) -> &Vec<HTMLRow> {
        &self.rows
    }

    fn get_mut_elements(&mut self) -> &mut Vec<HTMLRow> {
        &mut self.rows
    }
}
