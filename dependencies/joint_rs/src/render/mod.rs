use crate::core::JointApp;

/// JointRenderer will take any renderer in as long as it renders from a JointApp
pub trait JointRenderer<T> {
    type App: JointApp;
    fn render(&mut self, app: Self::App) -> T ;
}