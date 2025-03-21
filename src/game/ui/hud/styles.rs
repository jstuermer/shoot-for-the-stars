use crate::Color::Rgba;
use bevy::prelude::*;

pub const INFO_BAR_COLOR: Color = Rgba {
    red: 0.25,
    green: 0.25,
    blue: 0.25,
    alpha: 0.7,
}; // Color::DARK_GRAY with smaller alpha

pub const HUD_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.flex_direction = FlexDirection::Column;
    style
};

pub const INFO_BAR_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(8.0);
    style.height = Val::Percent(20.0);
    style.top = Val::Px(10.0);
    style.left = Val::Px(10.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.row_gap = Val::Px(10.0);
    style
};

pub const INFO_ITEM_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.left = Val::Px(10.0);
    style.column_gap = Val::Px(10.0);
    style
};

pub fn get_text_style(font_size: f32, asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size,
        color: Color::WHITE,
    }
}
