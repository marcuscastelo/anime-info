use std::collections::HashSet;

pub trait VecUtils<T> {
    fn into_unique(self) -> Vec<T>;
}

impl<T> VecUtils<T> for Vec<T>
where
    T: std::cmp::Eq + std::hash::Hash + Clone,
{
    fn into_unique<'a>(self) -> Vec<T> {
        let mut set = HashSet::new();
        let mut new_vec = Vec::new();

        for elem in self.into_iter() {
            if !set.contains(&elem) {
                set.insert(elem.clone());
                new_vec.push(elem);
            }
        }

        new_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_utils() {
        let vec = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        let unique_vec = vec.into_unique();
        assert_eq!(unique_vec, vec![1, 2, 3, 4, 5]);
    }
}
