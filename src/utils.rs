use bevy::window::Window;

pub fn get_confinement(window: &Window, character_size: f32) -> [f32; 4] {
    let half_character_size = character_size / 2.0;
    return [
        half_character_size,
        window.width() - half_character_size,
        half_character_size,
        window.height() - half_character_size,
    ];
}
