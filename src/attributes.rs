use crate::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    iter::IntoIterator,
};

#[derive(Clone)]
pub struct Attributes {
    style: HashMap<String, String>,
    class: HashSet<String>,
    id: String,
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            style: HashMap::new(),
            class: HashSet::new(),
            id: String::new(),
        }
    }

    pub fn get_styles(&self) -> &HashMap<String, String> {
        &self.style
    }

    pub fn get_style(&self, property: &String) -> Option<&String> {
        self.style.get(property)
    }

    pub fn set_style(&mut self, property: String, value: String) -> Option<String> {
        self.style.insert(property, value)
    }

    pub fn set_styles<T: IntoIterator<Item = (String, String)>>(&mut self, styles: T) {
        for (property, value) in styles {
            self.set_style(property, value);
        }
    }

    pub fn add_class(&mut self, class: String) -> bool {
        self.class.insert(class)
    }

    pub fn has_class(&mut self, class: &String) -> bool {
        self.class.contains(class)
    }

    pub fn remove_class(&mut self, class: &String) -> bool {
        self.class.remove(class)
    }

    pub fn toggle_class(&mut self, class: String) {
        if !self.remove_class(&class) {
            self.class.insert(class);
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
}

impl HTMLRendering for Attributes {
    fn render(&self) -> String {
        let style_str: String = self
            .style
            .iter()
            .flat_map(|(key, value)| {
                let s = format!("{}: {};", &key, &value);
                s.chars().collect::<Vec<_>>()
            })
            .collect();

        let class: String = self
            .class
            .iter()
            .flat_map(|s| s.chars().collect::<Vec<_>>())
            .collect();

        format!(
            "style=\"{}\" class=\"{}\" id=\"{}\"",
            style_str, class, &self.id
        )
    }
}
