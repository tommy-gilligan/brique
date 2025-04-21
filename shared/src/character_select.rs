use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};

pub async fn process<D>(device: &mut D) -> Option<&str>
where
    D: DrawTarget<Color = BinaryColor> + crate::Keypad,
{
    let mut characters = [
        "<", ">", "[", "]", "(", ")", "{", "}", "\\", "/", "=", "+", "*", "-", "\"", "'", "`", "@",
        "^", "#", "$", "%", "&", ",", ".", ":", ";", "!", "?", "_", "|", "~",
    ];
    let mut menu = crate::menu::Menu::new(&mut characters, Some("Use"), |a, b, c, d, e| {
        crate::menu::grid_render(a, b, c, d, e)
    });
    return menu.process(device).await;
}
