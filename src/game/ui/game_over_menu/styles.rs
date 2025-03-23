use bevy::prelude::*;

pub const GAME_OVER_MENU_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.width = Val::Percent(100.0);
    node.height = Val::Percent(100.0);
    node.flex_direction = FlexDirection::Column;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node
};

pub const INFO_ITEM_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.column_gap = Val::Px(20.0);
    node.flex_direction = FlexDirection::Row;
    node
};
