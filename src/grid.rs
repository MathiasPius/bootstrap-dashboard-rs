use std::{borrow::Cow, collections::HashSet, fmt::Display, hash::Hash};

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

#[derive(Template)]
#[template(path = "column.html")]
pub struct Column {
    sizes: HashSet<ColumnSize>,
    content: Cow<'static, str>,
}

impl Column {
    pub fn new<Content: Into<Cow<'static, str>>>(content: Content) -> Column {
        Column {
            sizes: HashSet::new(),
            content: content.into(),
        }
    }

    pub fn with_size<Width: Into<ColumnWidth>>(
        mut self,
        breakpoint: Breakpoint,
        width: Width,
    ) -> Self {
        self.sizes.insert(ColumnSize::new(breakpoint, width.into()));
        self
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for Column {
    fn from(value: T) -> Self {
        Column::new(value)
    }
}

#[derive(Template)]
#[template(path = "row.html")]
pub struct Row(Vec<Column>);

impl<'row> Row {
    pub fn new() -> Self {
        Row(Vec::new())
    }

    pub fn with_column<C: Into<Column>>(mut self, column: C) -> Self {
        self.0.push(column.into());
        self
    }
}
