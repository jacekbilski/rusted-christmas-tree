pub trait RenderLoopObserver {
    fn new() -> Self;
    fn tick(&mut self);
}
