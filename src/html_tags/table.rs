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

impl HTMLTable {
    pub fn new() -> Self {
        Self {
            head: Vec::new(),
            rows: Vec::new(),
            attributes: Attributes::new(),
        }
    }

    pub fn get_attributes(&self) -> &Attributes {
        &self.attributes
    }
    pub fn get_mut_attributes(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    pub fn with_row(row: HTMLRow) -> Self {
        Self {
            head: Vec::new(),
            rows: vec![row],
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

    pub fn get_head(&mut self) -> &mut Vec<HTMLRow> {
        &mut self.head
    }

    pub fn set_head(&mut self, row: Vec<HTMLRow>) {
        self.head = row;
    }
}

impl HTMLRendering for HTMLTable {
    fn render(&self) -> String {
        let mut output = format!("<table {}><tbody>", self.attributes.render());

        for row in &self.head {
            output.push_str(&row.render());
        }

        for row in &self.rows {
            output.push_str(&row.render());
        }

        output.push_str("</tbody></table>");

        output
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
