
// we can not use trait Extend with method extend_one because its nightly

pub trait Push<T> {
    fn push(& mut self, element:T);
}

impl<T> Push<T> for Vec<T> {
    fn push(& mut self, element:T) {
        self.push(element)
    }
}