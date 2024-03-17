pub struct Matrix<T> {
    _entries: Vec<Vec<T>>,
}

pub struct MatrixIterator<'a, T> {
    matrix: &'a Matrix<T>,
    index: usize,
}

impl<'a, T> Iterator for MatrixIterator<'a, &'a T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.matrix._entries.get(0)
    }
}

impl<T> Matrix<T> {
    pub fn new() -> Self {
        Self { _entries: vec![] }
    }
}
