#[derive(Debug, Clone)]
pub struct Array<T> {
    pub items: Vec<T>,
}

pub trait Length {
    fn length(&self) -> usize;
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

pub trait Pop {
    fn pop(&self) -> Self;
}

pub trait Push<T> {
    fn push(&self, value: T) -> Self;
}

impl<T: Clone> Push<T> for Array<T> {
    fn push(&self, value: T) -> Self {
        let mut new_items = self.items.clone();
        new_items.push(value);
        Array { items: new_items }
    }
}

impl<T> IsEmpty for Array<T> {
    fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
}

impl<T> Length for Array<T> {
    fn length(&self) -> usize {
        self.items.len()
    }
}

impl<T: Clone> Pop for Array<T> {
    fn pop(&self) -> Self {
        let mut new_items = self.items.clone();
        new_items.pop();
        Array { items: new_items }
    }
}
