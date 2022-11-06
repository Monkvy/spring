use sfml::graphics::Color;


pub fn tuple_to_color(tuple: (u8, u8, u8)) -> Color {
    Color::rgb(tuple.0, tuple.1, tuple.2)
}
