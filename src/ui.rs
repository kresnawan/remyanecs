#[derive(Debug)]
pub enum UIElement {
    UIButton,
    UIDiv,
    UISwitch,
    UIText,
    UISlot,
    UIRectangle
}

impl UIElement {
    pub fn t_index(&self) -> usize {
        match self {
            UIElement::UIDiv => 0,
            UIElement::UIButton => 1,
            UIElement::UISwitch => 2,
            UIElement::UIText => 3,
            UIElement::UISlot => 4,
            UIElement::UIRectangle => 5
        }
    }
}

#[derive(Debug)]
pub struct UILocation {
    pub table: UIElement,
    pub index: usize,
}