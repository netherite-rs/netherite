pub mod text_component;
pub mod style;
pub mod events;
pub mod component;
pub mod serializers;

#[cfg(test)]
mod tests {
    use crate::chat::style::RgbColor;
    use crate::chat::style::NamedTextColor;
    use crate::chat::text_component::TextComponent;

    #[test]
    fn test_rgb() {
        let color = RgbColor::new(10, 20, 30);
        let (red, green, blue) = color.rgb();
        assert_eq!(red, 10);
        assert_eq!(green, 20);
        assert_eq!(blue, 30);
    }

    #[test]
    fn test_json() {
        let component = TextComponent::builder()
            .text(String::from("Hello, world!"))
            .color(&NamedTextColor::Red)
            .bold()
            .build();
        println!("{}", serde_json::to_string(&component).unwrap());
    }
}