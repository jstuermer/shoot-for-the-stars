use bevy::prelude::*;

pub const NORMAL_BACKGROUND_COLOR: Color = Color::DARK_GRAY;

pub const NORMAL_BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style
};

pub const TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(600.0);
    style.height = Val::Px(150.0);
    style.flex_direction = FlexDirection::Row;
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style.column_gap = Val::Px(20.0);
    style
};

pub const MAIN_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.flex_direction = FlexDirection::Column;
    style.align_items = AlignItems::Center;
    style.align_self = AlignSelf::Center;
    style.justify_content = JustifyContent::Center;
    style.justify_self = JustifySelf::Center;
    style.row_gap = Val::Px(10.0);
    style
};

pub fn get_text_style(font_size: f32, asset_server: &Res<AssetServer>) -> TextStyle {
    return TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size,
        color: Color::WHITE,
    };
}
