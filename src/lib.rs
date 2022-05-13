use paste::paste;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

macro_rules! eziter {
    ($(
        $(($($template:ident),+))? $name:ident ( $($arg:ty),* ) -> $fret:ty => $col:ty,
    )*) => {

        pub trait EzRefIterator<'a>: RefIterator<'a> {
            $(
                fn $name<F, C $($(, $template)+)? >(&'a self, f: F) -> C
                where
                    Self: Sized,
                    F: FnMut($($arg),*) -> $fret,
                    C: std::iter::FromIterator<$col>,
                {
                    Iterator::$name(self.iter_ref(), f).collect()
                }
            )*
        }

        pub trait EzIntoIterator: IntoIterator {
            $(
                paste! {
                    fn [<into_ $name>]<F, C $($(, $template)+)? >(self, f: F) -> C
                    where
                        Self: Sized,
                        F: FnMut($($arg),*) -> $fret,
                        C: std::iter::FromIterator<$col>,
                    {
                        Iterator::$name(self.into_iter(), f).collect()
                    }
                }
            )*
        }

        pub trait EzMutIterator<'a>: MutIterator<'a> {
            $(
                paste! {
                    fn [<$name _mut>]<F, C $($(, $template)+)? >(&'a mut self, f: F) -> C
                    where
                        Self: Sized,
                        F: FnMut($($arg),*) -> $fret,
                        C: std::iter::FromIterator<$col>,
                    {
                        Iterator::$name(self.mut_iter(), f).collect()
                    }
                }
            )*
        }
    };
}

eziter! {
    (R) map(Self::Item) -> R => R,
    filter(&Self::Item) -> bool => Self::Item,
    filter_map(Self::Item) -> Option<Self::Item> => Self::Item,
    skip_while(&Self::Item) -> bool => Self::Item,
    take_while(&Self::Item) -> bool => Self::Item,
    (B) map_while(Self::Item) -> Option<B> => B,
}

pub trait RefIterator<'a> {
    type Item;

    fn iter_ref(&'a self) -> Box<dyn Iterator<Item = Self::Item> + '_>;
}

pub trait MutIterator<'a> {
    type Item;

    fn mut_iter(&'a mut self) -> Box<dyn Iterator<Item = Self::Item> + '_>;
}

macro_rules! impl_eziter_ref {
    ($(
        $name:ty{$($param:ident),*},
    )*) => {
        $(
            impl<'a, $($param: 'a),*> RefIterator<'a> for $name {
                #[allow(unused_parens)]
                type Item = ($(&'a $param),*);

                fn iter_ref(&'a self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
                    Box::new(self.iter())
                }
            }
        )*
    };
}

impl_eziter_ref! {
    HashMap<K, V>{K, V},
    BTreeMap<K, V>{K, V},
    HashSet<K>{K},
    BTreeSet<K>{K},
    BinaryHeap<T>{T},
    LinkedList<T>{T},
    Vec<T>{T},
    VecDeque<T>{T},
}

macro_rules! impl_eziter_mut {
    ($(
        $name:ty{$($param:ident),*}$item_mut:ty,
    )*) => {
        $(
            impl<'a, $($param: 'a),*> MutIterator<'a> for $name {
                #[allow(unused_parens)]
                type Item = $item_mut;

                fn mut_iter(&'a mut self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
                    Box::new(self.iter_mut())
                }
            }
        )*
    };
}

impl_eziter_mut! {
    HashMap<K, V>{K, V}(&'a K, &'a mut V),
    BTreeMap<K, V>{K, V}(&'a K, &'a mut V),
    LinkedList<T>{T}(&'a mut T),
    Vec<T>{T}(&'a mut T),
    VecDeque<T>{T}(&'a mut T),
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
        let v4: Vec<_> = v3.map_mut(|v| *v + 1);
        let v5: Vec<_> = v3.filter(|v| true);

        assert_eq!(h.get(&1), Some(&1));
        assert_eq!(h.get(&2), Some(&2));
    }
}
