use crate::core::Subject;

pub trait Cell {
    fn has_name(&self) -> Option<&String>;
    fn type_name(&self) -> &str;
    fn render_color_edit_mode(&self) -> &str;
    fn render_color_check_mode(&self) -> &str;
}

pub struct NormalCell {
    pub name: Option<String>,
}

pub struct TransparentCell;

pub struct BlockCell {
    pub name: Option<String>,
}

impl Cell for NormalCell {
    fn has_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    fn type_name(&self) -> &str {
        "Active"
    }

    fn render_color_edit_mode(&self) -> &str {
        "#e2e8f0"
    }

    fn render_color_check_mode(&self) -> &str {
        "#ffffff"
    }
}

impl Cell for TransparentCell {
    fn has_name(&self) -> Option<&String> {
        None
    }

    fn type_name(&self) -> &str {
        "Transparent"
    }

    fn render_color_edit_mode(&self) -> &str {
        "#475569"
    }

    fn render_color_check_mode(&self) -> &str {
        "transparent"
    }
}

impl Cell for BlockCell {
    fn has_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    fn type_name(&self) -> &str {
        "Block"
    }

    fn render_color_edit_mode(&self) -> &str {
        "#475569"
    }

    fn render_color_check_mode(&self) -> &str {
        "#475569"
    }
}

pub fn from_subject(subject: Option<&Subject>) -> Box<dyn Cell> {
    match subject {
        Some(Subject::Some(name)) => Box::new(NormalCell {
            name: Some(name.clone()),
        }),
        Some(Subject::Block(name)) => Box::new(BlockCell {
            name: Some(name.clone()),
        }),
        Some(Subject::Transparent) => Box::new(TransparentCell),
        None => Box::new(NormalCell { name: None }),
    }
}
