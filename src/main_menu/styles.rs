use crate::Srgba;
use bevy::color::palettes::css::DARK_GRAY;
use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Srgba = Srgba {
    red: 0.25,
    green: 0.25,
    blue: 0.25,
    alpha: 0.7,
}; // dark gray with smaller alpha
pub const HOVERED_BUTTON_COLOR: Srgba = DARK_GRAY;
pub const PRESSED_BUTTON_COLOR: Color = Color::BLACK;

pub const NORMAL_BUTTON_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.width = Val::Px(200.0);
    node.height = Val::Px(80.0);
    node.align_items = AlignItems::Center;
    node.justify_content = JustifyContent::Center;
    node
};

pub const TITLE_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.width = Val::Percent(100.0);
    node.height = Val::Px(150.0);
    node.flex_direction = FlexDirection::Row;
    node.align_items = AlignItems::Center;
    node.justify_content = JustifyContent::Center;
    node.column_gap = Val::Px(20.0);
    node
};

pub const MAIN_MENU_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.width = Val::Percent(100.0);
    node.height = Val::Percent(100.0);
    node.flex_direction = FlexDirection::Column;
    node.align_items = AlignItems::Center;
    node.align_self = AlignSelf::Center;
    node.justify_content = JustifyContent::Center;
    node.justify_self = JustifySelf::Center;
    node.row_gap = Val::Px(10.0);
    node
};
