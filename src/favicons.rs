use askama::Template;

#[derive(Debug, Clone, Template)]
#[template(
    ext = "html",
    source = r#"
{% match favicon %}
{% when Some with (favicon) %}
<link rel="icon" href="{{ favicon }}">
{% when None %}
{% endmatch %}
{% match apple_touch_icon %}
{% when Some with (apple_touch_icon) %}
<link rel="apple-touch-icon" href="{{ apple_touch_icon }}">
{% when None %}
{% endmatch %}
{% match mask_icon %}
{% when Some with (mask_icon) %}
<link rel="mask-icon" href="{{ mask_icon.href }}" color="{{ mask_icon.color }}">
{% when None %}
{% endmatch %}
"#
)]
pub struct FavIcons {
    pub favicon: Option<String>,
    pub apple_touch_icon: Option<String>,
    pub mask_icon: Option<MaskFavIcon>,
}

#[derive(Debug, Clone)]
pub struct MaskFavIcon {
    pub href: String,
    pub color: String,
}
