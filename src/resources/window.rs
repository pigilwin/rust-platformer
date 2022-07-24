use bevy::window::WindowDescriptor;

pub fn window_descriptor_builder() -> WindowDescriptor {
    return WindowDescriptor {
        title: "Platformer".to_string(),
        width: 640.0,
        height: 400.0,
        ..Default::default()
    };
}
