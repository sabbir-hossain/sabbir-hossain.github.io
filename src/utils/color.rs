use rand::seq::SliceRandom;
use rand::thread_rng;

pub const BACKGROUND_COLOR: [f32; 3] = [0.65, 0.65, 0.65];

pub enum Color {
    Red,
    Crimson,
    DeepPink,
    MediumVioletRed,
    OrangeRed,
    Orange,
    Gold,
    DarkKhaki,
    MediumOrchid,
    RebeccaPurple,
    Green,
    LimeGreen,
    ForestGreen,
    Olive,
    DarkCyan,
    Blue,
    SteelBlue,
    RoyalBlue,
    White,
    Azure,
    GhostWhite,
    MintCream,
    WhiteSmoke,
    Black,
    Gainsboro,
    DarkGray,
    Gray,
    RoadColor,
}

impl Color {
    pub fn get(&self) -> [f32; 3] {
        match self {
            Color::Red => [1.0, 0.0, 0.0],
            Color::Crimson => [0.86, 0.08, 0.24],
            Color::DeepPink => [1.0, 0.08, 0.58],
            Color::MediumVioletRed => [0.78, 0.08, 0.52],
            Color::OrangeRed => [1.0, 0.27, 0.0],
            Color::Orange => [1.0, 0.65, 0.0],
            Color::Gold => [1.0, 0.84, 0.0],
            Color::DarkKhaki => [0.74, 0.72, 0.42],
            Color::MediumOrchid => [0.73, 0.33, 0.83],
            Color::RebeccaPurple => [0.4, 0.2, 0.6],
            Color::Green => [0.0, 1.0, 0.0],
            Color::LimeGreen => [0.2, 0.8, 0.2],
            Color::ForestGreen => [0.13, 0.55, 0.13],
            Color::Olive => [0.5, 0.5, 0.0],
            Color::DarkCyan => [0.0, 0.55, 0.55],
            Color::Blue => [0.0, 0.0, 1.0],
            Color::SteelBlue => [0.27, 0.51, 0.71],
            Color::RoyalBlue => [0.25, 0.41, 0.88],
            Color::White => [1.0, 1.0, 1.0],
            Color::Azure => [0.94, 1.0, 1.0],
            Color::GhostWhite => [0.97, 0.97, 1.0],
            Color::MintCream => [0.96, 1.0, 0.98],
            Color::WhiteSmoke => [0.96, 0.96, 0.96],
            Color::Black => [0.0, 0.0, 0.0],
            Color::Gainsboro => [0.86, 0.86, 0.86],
            Color::DarkGray => [0.66, 0.66, 0.66],
            Color::Gray => [0.75, 0.75, 0.75],
            Color::RoadColor => [0.067, 0.067, 0.067], // Example road color
        }
    }
}

pub fn get_random_color() -> [f32; 3] {
    let colors = [
        Color::Red,
        Color::Crimson,
        Color::DeepPink,
        Color::MediumVioletRed,
        Color::OrangeRed,
        Color::Orange,
        Color::Gold,
        Color::DarkKhaki,
        Color::MediumOrchid,
        Color::RebeccaPurple,
        Color::Green,
        Color::LimeGreen,
        Color::ForestGreen,
        Color::Olive,
        Color::DarkCyan,
        Color::Blue,
        Color::SteelBlue,
        Color::RoyalBlue,
        Color::White,
        Color::Azure,
        Color::GhostWhite,
        Color::MintCream,
        Color::WhiteSmoke,
        Color::Gainsboro,
        Color::Gray,


        // Color::Black,
        // Color::DarkGray, // same color as background
    ];
    let color = colors.choose(&mut thread_rng()).unwrap(); 

    Color::get(&color)
}