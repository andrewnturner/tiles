pub struct Vec2d<T> {
    rows: Vec<Vec<T>>,
}

impl<T> Vec2d<T>
where
    T: Clone + Default,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            rows: vec![vec![T::default(); width]; height],
        }
    }
}

impl<T> Vec2d<T> {
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &Vec<Vec<T>> {
        &self.rows
    }
}
