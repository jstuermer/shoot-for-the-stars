use bevy::window::Window;

pub fn get_confinement(window: &Window, character_width: f32, character_height: f32) -> [f32; 4] {
    let half_character_width = character_width / 2.0;
    let half_character_height = character_height / 2.0;
    [
        half_character_width,
        window.width() - half_character_width,
        half_character_height,
        window.height() - half_character_height,
    ]
}
