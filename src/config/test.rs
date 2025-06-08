use crate::config::{FpsConfig, Position, Text};

#[test]
fn test_fps_config_0001() {
    let mut config = FpsConfig {
        enable: false,
        frame_limit: 30,
        config: Text {
            scale: 1.0,
            color: [0, 1, 2, 255],
            position: Position::Center,
        },
    };

    config.config.position = Position::TopLeft;
    assert!(config.is_valid_position());

    config.config.position = Position::TopRight;
    assert!(config.is_valid_position());

    config.config.position = Position::BottomLeft;
    assert!(config.is_valid_position());

    config.config.position = Position::BottomRight;
    assert!(config.is_valid_position());
}

#[test]
fn test_fps_config_0002() {
    let mut config = FpsConfig {
        enable: false,
        frame_limit: 30,
        config: Text {
            scale: 1.0,
            color: [0, 1, 2, 255],
            position: Position::Center,
        },
    };

    assert!(!config.is_valid_position());

    config.config.position = Position::Left;
    assert!(!config.is_valid_position());

    config.config.position = Position::Right;
    assert!(!config.is_valid_position());

    config.config.position = Position::Top;
    assert!(!config.is_valid_position());

    config.config.position = Position::Bottom;
    assert!(!config.is_valid_position());
}
