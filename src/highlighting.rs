use termion::color;

#[derive(PartialEq)]
pub enum Type {
    None,
    Number,
    Match,
}

impl Type {
    pub fn to_color(&self) -> Option<impl color::Color> {
        match self {
            Type::Number => Some(color::Rgb(220, 163, 163)),
            Type::Match => Some(color::Rgb(38, 139, 210)),
            _ => None,
        }
    }
}
