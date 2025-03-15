pub trait AbstractMenu {
    type Item;
    fn new(items: &[Self::Item]) -> Self;
    fn down(&mut self);
    fn up(&mut self);
    fn process(
        &mut self,
        device: &mut impl crate::Device,
    ) -> impl core::future::Future<Output = &Self::Item> + Send;
}
