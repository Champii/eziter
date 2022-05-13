use std::collections::HashMap;

trait EzIntoIterator: IntoIterator {
    fn into_map<F, R, C>(self, f: F) -> C
    where
        F: FnMut(<Self as IntoIterator>::Item) -> R,
        Self: Sized,
        C: std::iter::FromIterator<R>,
    {
        Iterator::map(self.into_iter(), f).collect()
    }

    fn filter<F, C>(self, f: F) -> C
    where
        F: FnMut(&<Self as IntoIterator>::Item) -> bool,
        Self: Sized,
        C: std::iter::FromIterator<<Self as IntoIterator>::Item>,
    {
        Iterator::filter(self.into_iter(), f).collect()
    }
}

trait EzRefIterator<'a>: RefIterator<'a> {
    fn map<F, R, C>(&'a self, f: F) -> C
    where
        F: FnMut(<Self as RefIterator>::Item) -> R,
        Self: Sized,
        C: std::iter::FromIterator<R>,
    {
        Iterator::map(self.iter_ref(), f).collect()
    }

    fn filter<F, C>(&'a self, f: F) -> C
    where
        F: FnMut(&<Self as RefIterator>::Item) -> bool,
        Self: Sized,
        C: std::iter::FromIterator<<Self as RefIterator<'a>>::Item>,
    {
        Iterator::filter(self.iter_ref(), f).collect()
    }
}

trait EzMutIterator<'a>: MutIterator<'a> {
    fn map_mut<F, R, C>(&'a mut self, f: F) -> C
    where
        F: FnMut(<Self as MutIterator>::Item) -> R,
        Self: Sized,
        C: std::iter::FromIterator<R>,
    {
        Iterator::map(self.mut_iter(), f).collect()
    }

    fn filter<F, C>(&'a mut self, f: F) -> C
    where
        F: FnMut(&<Self as MutIterator>::Item) -> bool,
        Self: Sized,
        C: std::iter::FromIterator<<Self as MutIterator<'a>>::Item>,
    {
        Iterator::filter(self.mut_iter(), f).collect()
    }
}

trait RefIterator<'a> {
    type Item;

    fn iter_ref(&'a self) -> Box<dyn Iterator<Item = Self::Item> + '_>;
}

impl<'a, K: 'a, V: 'a> RefIterator<'a> for HashMap<K, V> {
    type Item = (&'a K, &'a V);

    fn iter_ref(&'a self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.iter())
    }
}

impl<'a, T: 'a> RefIterator<'a> for Vec<T> {
    type Item = &'a T;

    fn iter_ref(&'a self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.iter())
    }
}

trait MutIterator<'a> {
    type Item;

    fn mut_iter(&'a mut self) -> Box<dyn Iterator<Item = Self::Item> + '_>;
}

impl<'a, K: 'a, V: 'a> MutIterator<'a> for HashMap<K, V> {
    type Item = (&'a K, &'a mut V);

    fn mut_iter(&'a mut self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.iter_mut())
    }
}

impl<'a, T: 'a> MutIterator<'a> for Vec<T> {
    type Item = &'a mut T;

    fn mut_iter(&'a mut self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.iter_mut())
    }
}

impl<T> EzIntoIterator for T where T: IntoIterator {}
impl<'a, T> EzRefIterator<'a> for T where T: RefIterator<'a> {}
impl<'a, T> EzMutIterator<'a> for T where T: MutIterator<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let v = vec![1, 2];
        let h: HashMap<_, _> = v.into_map(|x| (x, x));
        let v2: Vec<_> = h.map(|(k, v)| k + v);
        let mut v3: Vec<_> = v2.map(|v| v + 1);
        let _v4: Vec<_> = v3.map_mut(|v| *v + 1);

        assert_eq!(h.get(&1), Some(&1));
        assert_eq!(h.get(&2), Some(&2));
    }
}
