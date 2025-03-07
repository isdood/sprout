use palette::{Srgb, Hsv};
use rand::Rng;

pub struct Theme {
    primary: Srgb,
    secondary: Srgb,
    background: Srgb,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            primary: Self::generate_random_color(),
            secondary: Self::generate_random_color(),
            background: Self::generate_random_color(),
        }
    }

    fn generate_random_color() -> Srgb {
        let mut rng = rand::thread_rng();
        let hue: f32 = rng.gen_range(0.0..360.0);
        let saturation: f32 = rng.gen_range(0.5..1.0);
        let value: f32 = rng.gen_range(0.5..1.0);
        Hsv::new(hue, saturation, value).into()
    }

    pub fn update_theme(&mut self, primary: Option<Srgb>, secondary: Option<Srgb>, background: Option<Srgb>) {
        if let Some(color) = primary {
            self.primary = color;
        }
        if let Some(color) = secondary {
            self.secondary = color;
        }
        if let Some(color) = background {
            self.background = color;
        }
    }
}
