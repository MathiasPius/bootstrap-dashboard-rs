use std::{
    borrow::Cow,
    fmt::{Debug, Display, Write},
    time::Duration,
    vec,
};

use askama::Template;

/// Dynamic auto-updating wrapper around a template or object.
#[derive(Debug, Clone, Template)]
#[template(path = "dynamic.html")]
pub struct Dynamic<T: Display> {
    pub content: T,
    hx: Option<HxSwap>,
}

impl<T: Display> Dynamic<T> {
    pub fn new(value: T, hx: HxSwap) -> Self {
        Dynamic {
            content: value,
            hx: Some(hx),
        }
    }

    pub fn with_hx(self, hx: HxSwap) -> Self {
        Dynamic {
            content: self.content,
            hx: Some(hx),
        }
    }
}

impl<T: Display> From<T> for Dynamic<T> {
    fn from(value: T) -> Self {
        Dynamic {
            content: value,
            hx: None,
        }
    }
}

pub trait IntoDynamic: Display + Sized {
    fn with_hx(self, hx: HxSwap) -> Dynamic<Self>;
}

impl<T: Display> IntoDynamic for T {
    fn with_hx(self, hx: HxSwap) -> Dynamic<Self> {
        Dynamic {
            content: self,
            hx: Some(hx),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HxSwap {
    url: Request,
    pub target: Option<Target>,
    pub triggers: Vec<Trigger>,
    pub swap: Option<Swap>,
}

#[derive(Debug, Clone)]
enum Request {
    Get(Cow<'static, str>),
    Post(Cow<'static, str>),
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Request::Get(url) => write!(f, r#"hx-get="{url}""#),
            Request::Post(url) => write!(f, r#"hx-post="{url}""#),
        }
    }
}

impl HxSwap {
    pub fn get<T: Into<Cow<'static, str>>>(path: T) -> Self {
        HxSwap {
            url: Request::Get(path.into()),
            target: None,
            triggers: vec![],
            swap: None,
        }
    }

    pub fn post<T: Into<Cow<'static, str>>>(path: T) -> Self {
        HxSwap {
            url: Request::Post(path.into()),
            target: None,
            triggers: vec![],
            swap: None,
        }
    }

    pub fn with_target<T: Into<Target>>(mut self, target: T) -> Self {
        self.target.replace(target.into());
        self
    }

    pub fn with_trigger<T: Into<Trigger>>(mut self, trigger: T) -> Self {
        self.triggers.push(trigger.into());
        self
    }

    pub fn with_swap<T: Into<Swap>>(mut self, swap: T) -> Self {
        self.swap.replace(swap.into());
        self
    }
}

impl Display for HxSwap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)?;

        if let Some(target) = &self.target {
            write!(f, r#" hx-target="{target}""#)?;
        }

        if !self.triggers.is_empty() {
            write!(
                f,
                r#" hx-trigger="{}""#,
                self.triggers
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        if let Some(swap) = &self.swap {
            write!(f, "{}", swap)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScrollDirection {
    Top,
    Bottom,
}

impl AsRef<str> for ScrollDirection {
    fn as_ref(&self) -> &str {
        match self {
            ScrollDirection::Top => "top",
            ScrollDirection::Bottom => "bottom",
        }
    }
}

impl Display for ScrollDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TriggerEvent {
    Load,
    Click,
    MouseEnter,
    KeyUp,
    Revealed,
    Every(Duration),
}

impl TriggerEvent {
    pub fn with_conditional<T: Into<Cow<'static, str>>>(self, conditional: T) -> Trigger {
        Trigger::from(self).with_conditional(conditional)
    }

    pub fn with_modifier(self, modifier: EventModifier) -> Trigger {
        Trigger::from(self).with_modifier(modifier)
    }
}

impl Display for TriggerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TriggerEvent::Load => f.write_str("load"),
            TriggerEvent::Click => f.write_str("click"),
            TriggerEvent::MouseEnter => f.write_str("mouseenter"),
            TriggerEvent::KeyUp => f.write_str("keyup"),
            TriggerEvent::Revealed => f.write_str("revelead"),
            TriggerEvent::Every(timing) => write!(f, "every {timing:?}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Trigger {
    pub event: TriggerEvent,
    pub conditional: Option<Cow<'static, str>>,
    pub modifiers: Vec<EventModifier>,
}

impl Trigger {
    pub fn with_conditional<T: Into<Cow<'static, str>>>(mut self, conditional: T) -> Self {
        self.conditional.replace(conditional.into());
        self
    }

    pub fn with_modifier(mut self, modifier: EventModifier) -> Self {
        self.modifiers.push(modifier);
        self
    }
}

impl Display for Trigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.event)?;

        for modifier in &self.modifiers {
            f.write_char(' ')?;
            write!(f, "{}", modifier)?;
        }

        if let Some(expression) = &self.conditional {
            write!(f, " [{expression}]")?;
        }

        Ok(())
    }
}

impl From<TriggerEvent> for Trigger {
    fn from(value: TriggerEvent) -> Self {
        Trigger {
            event: value,
            conditional: None,
            modifiers: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum EventModifier {
    Once,
    Changed,
    Delay(Duration),
    Throttle(Duration),
    From(Cow<'static, str>),
    Target(Cow<'static, str>),
    Consume,
    Queue(QueueOption),
}

impl Display for EventModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventModifier::Once => f.write_str("once"),
            EventModifier::Changed => f.write_str("changed"),
            EventModifier::Delay(timing) => write!(f, "delay:{timing:?}"),
            EventModifier::Throttle(timing) => write!(f, "throttle:{timing:?}"),
            EventModifier::From(from) => write!(f, "from:{from}"),
            EventModifier::Target(target) => write!(f, "target:{target}"),
            EventModifier::Consume => f.write_str("consume"),
            EventModifier::Queue(option) => write!(f, "queue:{option}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QueueOption {
    First,
    Last,
    All,
    None,
}

impl AsRef<str> for QueueOption {
    fn as_ref(&self) -> &str {
        match self {
            QueueOption::First => "first",
            QueueOption::Last => "last",
            QueueOption::All => "all",
            QueueOption::None => "none",
        }
    }
}

impl Display for QueueOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Swap {
    pub target: SwapTarget,
    pub modifiers: Option<SwapModifiers>,
}

impl From<SwapTarget> for Swap {
    fn from(value: SwapTarget) -> Self {
        Swap {
            target: value,
            modifiers: None,
        }
    }
}

impl Display for Swap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"hx-swap="{}{}""#,
            self.target,
            self.modifiers
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone)]
pub struct SwapModifiers {
    pub transition: Option<bool>,
    pub swap_delay: Option<Duration>,
    pub settle_delay: Option<Duration>,
    pub ignore_title: Option<bool>,
    pub scroll: Option<ScrollDirection>,
    pub show: Option<ScrollDirection>,
}

impl Display for SwapModifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(transition) = self.transition {
            write!(f, " transition:{transition}")?;
        }

        if let Some(swap_delay) = self.swap_delay {
            write!(f, " swap:{swap_delay:?}")?;
        }

        if let Some(settle_delay) = self.settle_delay {
            write!(f, " settle:{settle_delay:?}")?;
        }

        if let Some(ignore_title) = self.ignore_title {
            write!(f, " ignoreTitle:{ignore_title}")?;
        }

        if let Some(scroll) = self.scroll {
            write!(f, " scroll:{scroll}")?;
        }

        if let Some(show) = self.show {
            write!(f, " show:{show}")?;
        }

        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SwapTarget {
    #[default]
    InnerHtml,
    OuterHtml,
    BeforeBegin,
    AfterBegin,
    BeforeEnd,
    AfterEnd,
    Delete,
    None,
}

#[derive(Debug, Clone)]
pub enum Target {
    This,
    Closest(Cow<'static, str>),
    Find(Cow<'static, str>),
    Next(Cow<'static, str>),
    Previous(Cow<'static, str>),
    Specific(Cow<'static, str>),
}

impl From<&'static str> for Target {
    fn from(value: &'static str) -> Self {
        Target::Specific(value.into())
    }
}

impl From<String> for Target {
    fn from(value: String) -> Self {
        Target::Specific(value.into())
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::This => f.write_str("this"),
            Target::Closest(value) => write!(f, "closest:{value}"),
            Target::Find(value) => write!(f, "find:{value}"),
            Target::Next(value) => write!(f, "next:{value}"),
            Target::Previous(value) => write!(f, "previous:{value}"),
            Target::Specific(value) => f.write_str(value),
        }
    }
}

impl AsRef<str> for SwapTarget {
    fn as_ref(&self) -> &str {
        match self {
            SwapTarget::InnerHtml => "innerHTML",
            SwapTarget::OuterHtml => "outerHTML",
            SwapTarget::BeforeBegin => "beforebegin",
            SwapTarget::AfterBegin => "afterbegin",
            SwapTarget::BeforeEnd => "beforeend",
            SwapTarget::AfterEnd => "afterend",
            SwapTarget::Delete => "delete",
            SwapTarget::None => "none",
        }
    }
}

impl Display for SwapTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

#[cfg(test)]
#[test]
pub fn hx_construction() {
    let props = HxSwap::get("/notifications")
        .with_target("div#lol")
        .with_trigger(TriggerEvent::Every(Duration::from_millis(1500)).with_conditional("ctrlKey"))
        .with_trigger(
            TriggerEvent::Click
                .with_modifier(EventModifier::Delay(Duration::from_millis(2500)))
                .with_modifier(EventModifier::Changed)
                .with_modifier(EventModifier::Consume),
        )
        .to_string();

    println!("{}", props);
}
