use crate::Srgba;
use bevy::prelude::*;

pub const INFO_BAR_COLOR: Srgba = Srgba {
    red: 0.25,
    green: 0.25,
    blue: 0.25,
    alpha: 0.7,
}; // Color::DARK_GRAY with smaller alpha

pub const HUD_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.width = Val::Percent(100.0);
    node.height = Val::Percent(100.0);
    node.flex_direction = FlexDirection::Column;
    node
};

pub const INFO_BAR_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.width = Val::Percent(8.0);
    node.height = Val::Percent(20.0);
    node.top = Val::Px(10.0);
    node.left = Val::Px(10.0);
    node.flex_direction = FlexDirection::Column;
    node.align_items = AlignItems::Center;
    node.row_gap = Val::Px(10.0);
    node
};

pub const INFO_ITEM_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.top = Val::Px(5.0);
    node.column_gap = Val::Px(10.0);
    node
};
