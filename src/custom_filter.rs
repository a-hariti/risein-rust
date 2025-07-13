struct FilterCondition<T: PartialEq> {
    value: T,
}
impl<T: PartialEq> FilterCondition<T> {
    pub fn is_match(&self, item: &T) -> bool {
        self.value == *item
    }
    pub fn new(value: T) -> Self {
        FilterCondition { value }
    }
}

fn custom_filter<T: PartialEq>(collection: Vec<T>, condition: &FilterCondition<T>) -> Vec<T> {
    collection
        .into_iter()
        .filter(|item| condition.is_match(item))
        .collect()
}

pub fn main() {
    let collection = vec![1, 2, 3, 4, 5];
    let condition = FilterCondition::new(3);
    let filtered = custom_filter(collection, &condition);
    println!("{:?}", filtered);
}
