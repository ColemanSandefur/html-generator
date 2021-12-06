mod paragraph;

pub use paragraph::*;

#[derive(Clone)]
pub enum HTMLElement {
    Table(HTMLTable),
    Row(HTMLRow),
    Paragraph(HTMLParagraph),
}

impl HTMLRendering for HTMLElement {
    fn render(&self) -> String {
        match self {
            HTMLElement::Table(table) => table.render(),
            HTMLElement::Row(row) => row.render(),
            HTMLElement::Paragraph(p) => p.render(),
        }
    }
}

pub trait HTMLRendering {
    fn render(&self) -> String;
}

#[derive(Clone)]
pub struct HTMLRow {
    row: Vec<HTMLElement>,
}

impl HTMLRendering for HTMLRow {
    fn render(&self) -> String {
        let mut output = String::new();

        output.push_str("<tr>");

        for element in &self.row {
            output.push_str(format!("<td>{}</td>", element.render()).as_str());
        }

        output.push_str("</tr>");

        output
    }
}

impl HTMLRow {
    pub fn new() -> Self {
        Self { row: Vec::new() }
    }

    pub fn push(&mut self, element: HTMLElement) {
        self.row.push(element);
    }
}
impl From<HTMLRow> for HTMLElement {
    fn from(row: HTMLRow) -> Self {
        HTMLElement::Row(row)
    }
}

#[derive(Clone)]
pub struct HTMLTable {
    rows: Vec<HTMLRow>,
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
        Self { rows: Vec::new() }
    }

    pub fn push(&mut self, row: HTMLRow) {
        self.rows.push(row);
    }
}

impl From<HTMLTable> for HTMLElement {
    fn from(t: HTMLTable) -> Self {
        HTMLElement::Table(t)
    }
}
