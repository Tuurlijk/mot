use crate::model::{Appearance, Mode};
use ratatui::style::{Color, Modifier, Style};
use supports_color::{ColorLevel, Stream};

/// Initialize colors for the application
pub fn setup_colors(appearance: &mut Appearance) {
    // Detect terminal color support
    appearance.color_support = supports_color::on(Stream::Stdout);

    // Set color mode and foreground colors based on terminal background
    set_color_preferences(
        &mut appearance.color_mode,
        &mut appearance.default_foreground_color,
        &mut appearance.default_foreground_color_indexed,
        &mut appearance.default_foreground_color_dimmed,
        &mut appearance.default_foreground_color_dimmed_indexed,
    );

    // Set default style using the detected colors
    appearance.default_style = Style::default().fg(Color::Indexed(rgb_to_indexed(
        appearance.default_foreground_color,
    )));
}

// Helper to calculate a gradient color based on distance from selected row
pub fn gradient_color(
    distance: usize,
    selected: bool,
    color_level: Option<ColorLevel>,
    default_foreground: (u8, u8, u8),
    color_mode: Mode,
) -> Style {
    // If this is the selected row, use reversed style
    if selected {
        return Style::default().add_modifier(Modifier::REVERSED | Modifier::ITALIC);
    }

    // For terminals with no color support, just return default style
    if color_level.is_none() {
        return Style::default();
    }

    // No effect for selected row and immediate neighbors
    if distance == 0 {
        return Style::default();
    }

    // Maximum distance for gradient effect
    let max_distance = 20;

    // Calculate progress using a sine wave function instead of linear progression
    // Map distance to a value between 0 and PI/2 (0 to 90 degrees)
    let normalized_distance = (distance as f32).min(max_distance as f32) / max_distance as f32;
    let progress = (normalized_distance * std::f32::consts::PI / 2.0).sin();

    // Apply sine wave gradient based on terminal capabilities
    let foreground = default_foreground;

    // Calculate dimmed foreground color based on color mode
    let dimmed = calculate_dimmed_color(foreground, color_mode);

    // Calculate interpolated color with proper clamping based on color mode
    let color = interpolate_color(foreground, dimmed, progress, color_mode);

    // Create style with the calculated color
    match color_level {
        Some(level) if level.has_16m => {
            // For truecolor terminals, use RGB directly
            Style::default().fg(Color::Rgb(color.0, color.1, color.2))
        }
        Some(level) if level.has_256 => {
            // For 256-color terminals, convert to indexed color
            Style::default().fg(Color::Indexed(rgb_to_indexed(color)))
        }
        _ => {
            // For basic terminals, use simple dimming
            if progress > 0.5 {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default()
            }
        }
    }
}

// Calculate dimmed foreground color based on color mode
pub fn calculate_dimmed_color(foreground: (u8, u8, u8), color_mode: Mode) -> (u8, u8, u8) {
    match color_mode {
        Mode::Dark => (
            (foreground.0 as f32 * 0.5).clamp(0.0, 255.0) as u8,
            (foreground.1 as f32 * 0.5).clamp(0.0, 255.0) as u8,
            (foreground.2 as f32 * 0.5).clamp(0.0, 255.0) as u8,
        ),
        Mode::Light => (
            (foreground.0 as f32 * 2.0).clamp(0.0, 255.0) as u8,
            (foreground.1 as f32 * 2.0).clamp(0.0, 255.0) as u8,
            (foreground.2 as f32 * 2.0).clamp(0.0, 255.0) as u8,
        ),
        _ => (
            (foreground.0 as f32 * 0.75).clamp(0.0, 255.0) as u8,
            (foreground.1 as f32 * 0.75).clamp(0.0, 255.0) as u8,
            (foreground.2 as f32 * 0.75).clamp(0.0, 255.0) as u8,
        ),
    }
}

// Interpolate between foreground and dimmed colors based on progress
pub fn interpolate_color(
    foreground: (u8, u8, u8),
    dimmed: (u8, u8, u8),
    progress: f32,
    color_mode: Mode,
) -> (u8, u8, u8) {
    let r = interpolate_component(foreground.0, dimmed.0, progress, color_mode);
    let g = interpolate_component(foreground.1, dimmed.1, progress, color_mode);
    let b = interpolate_component(foreground.2, dimmed.2, progress, color_mode);
    (r, g, b)
}

// Interpolate a single color component with proper clamping based on color mode
pub fn interpolate_component(fg: u8, dimmed: u8, progress: f32, color_mode: Mode) -> u8 {
    let value = fg as f32 + ((dimmed as f32 - fg as f32) * progress);

    // Clamp the value based on color mode
    let clamped = match color_mode {
        Mode::Dark => value.clamp(dimmed as f32, fg as f32),
        _ => value.clamp(fg as f32, dimmed as f32),
    };

    clamped as u8
}

// Helper function to convert hex color to (r,g,b) tuple
pub fn hex_to_rgb(hex: u32) -> (u8, u8, u8) {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    (r, g, b)
}

// Convert RGB values to an indexed color (16-231)
pub fn rgb_to_indexed(rgb: (u8, u8, u8)) -> u8 {
    // Convert RGB to the 6x6x6 color cube (0-5 for each component)
    let r_index = (rgb.0 as f32 / 256.0 * 6.0) as u8;
    let g_index = (rgb.1 as f32 / 256.0 * 6.0) as u8;
    let b_index = (rgb.2 as f32 / 256.0 * 6.0) as u8;

    // Ensure indices are in 0-5 range
    let r_idx = r_index.min(5);
    let g_idx = g_index.min(5);
    let b_idx = b_index.min(5);

    // Calculate the indexed color (16-231)
    16 + 36 * r_idx + 6 * g_idx + b_idx
}

// Set color preferences based on terminal background
pub fn set_color_preferences(
    color_mode: &mut Mode,
    default_foreground_color: &mut (u8, u8, u8),
    default_foreground_color_indexed: &mut Color,
    default_foreground_color_dimmed: &mut (u8, u8, u8),
    default_foreground_color_dimmed_indexed: &mut Color,
) {
    match terminal_light::luma() {
        Ok(luma) if luma > 0.85 => {
            // Light mode: use a dark gray (#333333)
            *color_mode = Mode::Light;
            *default_foreground_color = hex_to_rgb(0x333333);
            *default_foreground_color_indexed =
                Color::Indexed(rgb_to_indexed(*default_foreground_color));
            *default_foreground_color_dimmed =
                calculate_dimmed_color(*default_foreground_color, *color_mode);
            *default_foreground_color_dimmed_indexed =
                Color::Indexed(rgb_to_indexed(*default_foreground_color_dimmed));
        }
        Ok(luma) if luma < 0.2 => {
            // Dark mode: use a light gray (#C0C0C0)
            *color_mode = Mode::Dark;
            *default_foreground_color = hex_to_rgb(0xC0C0C0);
            *default_foreground_color_indexed =
                Color::Indexed(rgb_to_indexed(*default_foreground_color));
            *default_foreground_color_dimmed =
                calculate_dimmed_color(*default_foreground_color, *color_mode);
            *default_foreground_color_dimmed_indexed =
                Color::Indexed(rgb_to_indexed(*default_foreground_color_dimmed));
        }
        _ => {
            // Default to dark mode
            *color_mode = Mode::Dark;
            *default_foreground_color = hex_to_rgb(0xC0C0C0);
            *default_foreground_color_indexed =
                Color::Indexed(rgb_to_indexed(*default_foreground_color));
            *default_foreground_color_dimmed =
                calculate_dimmed_color(*default_foreground_color, *color_mode);
            *default_foreground_color_dimmed_indexed =
                Color::Indexed(rgb_to_indexed(*default_foreground_color_dimmed));
        }
    }
}
