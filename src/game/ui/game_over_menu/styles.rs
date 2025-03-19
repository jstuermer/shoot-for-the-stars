use bevy::prelude::*;

pub const GAME_OVER_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style
};

pub const INFO_ITEM_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.column_gap = Val::Px(20.0);
    style.flex_direction = FlexDirection::Row;
    style.align_self = AlignSelf::Center;
    style
};

pub fn get_text_style(font_size: f32, asset_server: &Res<AssetServer>) -> TextStyle {
    return TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size,
        color: Color::WHITE,
    };
}
