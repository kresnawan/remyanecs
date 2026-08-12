#[derive(Debug)]
pub enum UIElement {
    UIButton,
    UIDiv,
}

impl UIElement {
    pub fn t_index(&self) -> usize {
        match self {
            UIElement::UIButton => 1,

            UIElement::UIDiv => 0,
        }
    }
}

#[derive(Debug)]
pub struct UILocation {
    pub table: UIElement,
    pub index: usize,
}
