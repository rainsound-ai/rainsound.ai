use image::Rgba;

pub trait RgbaU8Extension {
    fn to_css_string(&self) -> String;
}

impl RgbaU8Extension for Rgba<u8> {
    fn to_css_string(&self) -> String {
        let [red, green, blue, alpha] = self.0;
        format!("rgba({}, {}, {}, {})", red, green, blue, alpha)
    }
}
