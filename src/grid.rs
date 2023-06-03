use std::{collections::HashSet, fmt::Display, hash::Hash};

use askama::Template;

#[derive(Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Breakpoint {
    #[default]
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl Display for Breakpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Breakpoint::ExtraSmall => "",
            Breakpoint::Small => "-sm",
            Breakpoint::Medium => "-md",
            Breakpoint::Large => "-lg",
            Breakpoint::ExtraLarge => "-xl",
        })
    }
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColumnWidth {
    #[default]
    None,
    Sized(u8),
    Auto,
}

impl From<u8> for ColumnWidth {
    fn from(value: u8) -> Self {
        ColumnWidth::Sized(value)
    }
}

impl Display for ColumnWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnWidth::None => Ok(()),
            ColumnWidth::Sized(n) => write!(f, "-{n}"),
            ColumnWidth::Auto => f.write_str("-auto"),
        }
    }
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColumnSize {
    pub breakpoint: Breakpoint,
    pub width: ColumnWidth,
}

impl Hash for ColumnSize {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.breakpoint.hash(state);
    }
}

impl ColumnSize {
    pub fn new(breakpoint: Breakpoint, width: ColumnWidth) -> Self {
        ColumnSize { breakpoint, width }
    }

    pub fn with_breakpoint(self, breakpoint: Breakpoint) -> Self {
        ColumnSize {
            breakpoint,
            width: self.width,
        }
    }

    pub fn with_width(self, width: u8) -> Self {
        ColumnSize {
            breakpoint: self.breakpoint,
            width: ColumnWidth::Sized(width),
        }
    }

    pub fn with_auto_width(self) -> Self {
        ColumnSize {
            breakpoint: self.breakpoint,
            width: ColumnWidth::Auto,
        }
    }
}

impl Display for ColumnSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "col{}{}", self.breakpoint, self.width)
    }
}

pub trait Column: Display {
    fn columns(&self) -> &HashSet<ColumnSize>;

    fn column_classes(&self) -> String {
        if self.columns().is_empty() {
            String::from("col")
        } else {
            self.columns()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        }
    }
}

#[derive(Template)]
#[template(path = "row.html")]
pub struct Row(Vec<Box<dyn Column>>);

impl<'row> Row {
    pub fn new() -> Self {
        Row(Vec::new())
    }

    pub fn add_column<C: Column + 'static>(mut self, column: C) -> Self {
        self.0.push(Box::new(column));
        self
    }
}
