use embedded_graphics::{
    Pixel,
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget},
    primitives::Rectangle,
};

impl DrawTarget for super::Device {
    type Color = BinaryColor;

    type Error = ();

    fn draw_iter<I: IntoIterator<Item = Pixel<<Self as DrawTarget>::Color>>>(
        &mut self,
        i: I,
    ) -> Result<(), <Self as DrawTarget>::Error> {
        let a = self.display.draw_iter(i);
        let _ = self.display.flush();
        a.map_err(|_| ())
    }
}

impl Dimensions for super::Device {
    fn bounding_box(&self) -> Rectangle {
        self.display.bounding_box()
    }
}
