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

    appearance.default_style = Style::default().fg(appearance.default_foreground_color_indexed);

    appearance.default_block = appearance
        .default_block
        .clone()
        .border_style(Style::default().fg(appearance.default_foreground_color_dimmed_indexed));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        // Test with common colors
        assert_eq!(
            hex_to_rgb(0xFF0000),
            (255, 0, 0),
            "Red color conversion failed"
        );
        assert_eq!(
            hex_to_rgb(0x00FF00),
            (0, 255, 0),
            "Green color conversion failed"
        );
        assert_eq!(
            hex_to_rgb(0x0000FF),
            (0, 0, 255),
            "Blue color conversion failed"
        );
        assert_eq!(
            hex_to_rgb(0xFFFFFF),
            (255, 255, 255),
            "White color conversion failed"
        );
        assert_eq!(
            hex_to_rgb(0x000000),
            (0, 0, 0),
            "Black color conversion failed"
        );

        // Test with mixed colors
        assert_eq!(
            hex_to_rgb(0xC0C0C0),
            (192, 192, 192),
            "Light gray conversion failed"
        );
        assert_eq!(
            hex_to_rgb(0x333333),
            (51, 51, 51),
            "Dark gray conversion failed"
        );
        assert_eq!(
            hex_to_rgb(0x800080),
            (128, 0, 128),
            "Purple conversion failed"
        );
    }

    #[test]
    fn test_rgb_to_indexed() {
        // Test color cube corners (should map to corners of the 6x6x6 color cube)
        assert_eq!(
            rgb_to_indexed((0, 0, 0)),
            16,
            "Black indexed color incorrect"
        );
        assert_eq!(
            rgb_to_indexed((255, 0, 0)),
            196,
            "Red indexed color incorrect"
        );
        assert_eq!(
            rgb_to_indexed((0, 255, 0)),
            46,
            "Green indexed color incorrect"
        );
        assert_eq!(
            rgb_to_indexed((0, 0, 255)),
            21,
            "Blue indexed color incorrect"
        );
        assert_eq!(
            rgb_to_indexed((255, 255, 0)),
            226,
            "Yellow indexed color incorrect"
        );
        assert_eq!(
            rgb_to_indexed((255, 0, 255)),
            201,
            "Magenta indexed color incorrect"
        );
        assert_eq!(
            rgb_to_indexed((0, 255, 255)),
            51,
            "Cyan indexed color incorrect"
        );
        assert_eq!(
            rgb_to_indexed((255, 255, 255)),
            231,
            "White indexed color incorrect"
        );

        // Test some intermediate colors
        assert_eq!(
            rgb_to_indexed((128, 128, 128)),
            145,
            "Gray indexed color incorrect"
        );
        assert_eq!(
            rgb_to_indexed((192, 192, 192)),
            188,
            "Light gray indexed color incorrect"
        );
    }

    #[test]
    fn test_calculate_dimmed_color() {
        // Test in dark mode
        let dimmed_dark = calculate_dimmed_color((200, 200, 200), Mode::Dark);
        assert_eq!(dimmed_dark, (100, 100, 100), "Dark mode dimming incorrect");

        // Test in light mode
        let dimmed_light = calculate_dimmed_color((100, 100, 100), Mode::Light);
        assert_eq!(
            dimmed_light,
            (200, 200, 200),
            "Light mode dimming incorrect"
        );

        // Test with color saturation in light mode (shouldn't exceed 255)
        let dimmed_light_saturated = calculate_dimmed_color((200, 200, 200), Mode::Light);
        assert_eq!(
            dimmed_light_saturated,
            (255, 255, 255),
            "Light mode saturation handling incorrect"
        );

        // Test with dark mode black color (shouldn't go below 0)
        let dimmed_dark_black = calculate_dimmed_color((0, 0, 0), Mode::Dark);
        assert_eq!(
            dimmed_dark_black,
            (0, 0, 0),
            "Dark mode black handling incorrect"
        );
    }

    #[test]
    fn test_interpolate_color() {
        // Test with no progress (should be the foreground color)
        let no_progress = interpolate_color((100, 100, 100), (50, 50, 50), 0.0, Mode::Dark);
        assert_eq!(
            no_progress,
            (100, 100, 100),
            "No progress interpolation incorrect"
        );

        // Test with full progress (should be the dimmed color)
        let full_progress = interpolate_color((100, 100, 100), (50, 50, 50), 1.0, Mode::Dark);
        assert_eq!(
            full_progress,
            (50, 50, 50),
            "Full progress interpolation incorrect"
        );

        // Test with half progress
        let half_progress = interpolate_color((100, 100, 100), (50, 50, 50), 0.5, Mode::Dark);
        assert_eq!(
            half_progress,
            (75, 75, 75),
            "Half progress interpolation incorrect"
        );

        // Test light mode interpolation
        let light_half_progress =
            interpolate_color((100, 100, 100), (200, 200, 200), 0.5, Mode::Light);
        assert_eq!(
            light_half_progress,
            (150, 150, 150),
            "Light mode half progress interpolation incorrect"
        );
    }

    #[test]
    fn test_interpolate_component() {
        // Test interpolation in dark mode
        assert_eq!(
            interpolate_component(100, 50, 0.0, Mode::Dark),
            100,
            "Dark mode 0.0 progress incorrect"
        );
        assert_eq!(
            interpolate_component(100, 50, 0.5, Mode::Dark),
            75,
            "Dark mode 0.5 progress incorrect"
        );
        assert_eq!(
            interpolate_component(100, 50, 1.0, Mode::Dark),
            50,
            "Dark mode 1.0 progress incorrect"
        );

        // Test interpolation in light mode
        assert_eq!(
            interpolate_component(100, 200, 0.0, Mode::Light),
            100,
            "Light mode 0.0 progress incorrect"
        );
        assert_eq!(
            interpolate_component(100, 200, 0.5, Mode::Light),
            150,
            "Light mode 0.5 progress incorrect"
        );
        assert_eq!(
            interpolate_component(100, 200, 1.0, Mode::Light),
            200,
            "Light mode 1.0 progress incorrect"
        );

        // Test clamping in dark mode (should not go below dimmed)
        assert_eq!(
            interpolate_component(100, 50, -0.5, Mode::Dark),
            100,
            "Dark mode negative progress clamping incorrect"
        );
        assert_eq!(
            interpolate_component(100, 50, 1.5, Mode::Dark),
            50,
            "Dark mode excessive progress clamping incorrect"
        );

        // Test clamping in light mode (should not go below foreground)
        assert_eq!(
            interpolate_component(100, 200, -0.5, Mode::Light),
            100,
            "Light mode negative progress clamping incorrect"
        );
        assert_eq!(
            interpolate_component(100, 200, 1.5, Mode::Light),
            200,
            "Light mode excessive progress clamping incorrect"
        );
    }

    #[test]
    fn test_gradient_color_selection() {
        // Test selected row (should be reversed style regardless of distance)
        let selected_style = gradient_color(5, true, None, (255, 255, 255), Mode::Dark);
        assert!(
            selected_style.add_modifier.contains(Modifier::REVERSED),
            "Selected row should have REVERSED style"
        );
        assert!(
            selected_style.add_modifier.contains(Modifier::ITALIC),
            "Selected row should have ITALIC style"
        );
    }

    #[test]
    fn test_gradient_color_immediate_neighbor() {
        // Test immediate neighbor (distance = 0, should be default style)
        let neighbor_style = gradient_color(0, false, None, (255, 255, 255), Mode::Dark);
        assert_eq!(
            neighbor_style,
            Style::default(),
            "Immediate neighbor should have default style"
        );
    }

    #[test]
    fn test_gradient_color_no_color_support() {
        // Test no color support (should return default style)
        let no_color_style = gradient_color(5, false, None, (255, 255, 255), Mode::Dark);
        assert_eq!(
            no_color_style,
            Style::default(),
            "No color support should return default style"
        );
    }
}
