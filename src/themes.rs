use tui::style::Color;

pub struct Theme {
    pub title_color: Color,
    pub dark_square_color: Color,
    pub light_square_color: Color,
    pub highlighted_color: Color,
    pub error_color: Color,
    pub text_color: Color,
    pub light_number_color: Color,
    pub dark_number_color: Color,
    pub victory_color: Color,
}

#[allow(dead_code)]
pub const TRANQUIL: Theme = Theme {
    title_color: Color::Rgb(170, 143, 121), // Pastel Brown
    dark_square_color: Color::Gray,
    light_square_color: Color::White,
    highlighted_color: Color::Rgb(184, 255, 184), // Mint
    error_color: Color::Rgb(255, 165, 161),       // Pastel Red
    text_color: Color::White,
    light_number_color: Color::Black,
    dark_number_color: Color::Black,
    victory_color: Color::Rgb(184, 255, 184), // Mint
};

// https://github.com/dracula/dracula-theme
#[allow(dead_code)]
pub const DRACULA: Theme = Theme {
    title_color: Color::Rgb(181, 147, 249),
    dark_square_color: Color::Rgb(68, 71, 90),
    light_square_color: Color::Rgb(40, 42, 54),
    highlighted_color: Color::Rgb(189, 147, 249),
    error_color: Color::Rgb(255, 85, 85),
    text_color: Color::Rgb(248, 248, 242),
    light_number_color: Color::Rgb(248, 248, 242),
    dark_number_color: Color::Rgb(248, 248, 242),
    victory_color: Color::Rgb(80, 250, 123),
};

#[allow(dead_code)]
pub const ONEDARK: Theme = Theme {
    title_color: Color::Rgb(198, 120, 221),
    dark_square_color: Color::Rgb(33, 37, 43),
    light_square_color: Color::Rgb(40, 44, 52),
    highlighted_color: Color::Rgb(98, 174, 238),
    error_color: Color::Rgb(224, 106, 116),
    text_color: Color::Rgb(171, 178, 191),
    light_number_color: Color::Rgb(171, 178, 191),
    dark_number_color: Color::Rgb(171, 178, 191),
    victory_color: Color::Rgb(151, 194, 121),
};

#[cfg(feature = "tranquil")]
pub const BOARD_THEME: Theme = TRANQUIL;
#[cfg(feature = "dracula")]
pub const BOARD_THEME: Theme = DRACULA;
#[cfg(feature = "onedark")]
pub const BOARD_THEME: Theme = ONEDARK;
