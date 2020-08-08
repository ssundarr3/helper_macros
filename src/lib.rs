/// A macro to create a HashSet from a list of values.
#[macro_export]
macro_rules! hashset {
    ($($expression:expr),*) => {{
        #[allow(unused_mut)]
        let mut s = ::std::collections::HashSet::with_capacity(num_items![$($expression),*]);
        $(s.insert($expression);)*
        s
    }};
}

/// A macro to create a HashMap from a list of key-value pairs.
#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {{
        #[allow(unused_mut)]
        let mut s = ::std::collections::HashMap::with_capacity(num_items![$($key),*]);
        $(s.insert($key, $value);)*
        s
    }};
}

/// A macro that simply replaces the first item with the second.
#[macro_export]
#[doc(hidden)]
macro_rules! replace {
    ($_expression:expr, $sub:expr) => {{
        $sub
    }};
}

/// A macro to count the number of items in a list.
#[macro_export]
#[doc(hidden)]
macro_rules! num_items {
    ($($expression:expr),*) => {{
        <[()]>::len(&[$(replace!($expression, ()),)* ])
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn hashset_empty() {
        let s: HashSet<()> = hashset![];
        assert!(s.is_empty());
    }

    #[test]
    fn hashset_nonempty() {
        let s = hashset![1, 2, 3];
        assert_eq!(s.len(), 3);
        assert!(s.contains(&1));
        assert!(s.contains(&2));
        assert!(s.contains(&3));
    }

    #[test]
    fn hashset_repeated_elements() {
        let s = hashset![1, 1, 2, 1, 3, 2];
        assert_eq!(s.len(), 3);
        assert!(s.contains(&1));
        assert!(s.contains(&2));
        assert!(s.contains(&3));
    }

    #[test]
    fn hashmap_empty() {
        let s: HashMap<(), ()> = hashmap![];
        assert!(s.is_empty());
    }

    #[test]
    fn hashmap_nonempty() {
        let m = hashmap![1 => "hello", 2 => "world", 3 => "!"];
        assert_eq!(m.len(), 3);
        assert_eq!(m.get(&1), Some(&"hello"));
        assert_eq!(m.get(&2), Some(&"world"));
        assert_eq!(m.get(&3), Some(&"!"));
    }

    #[test]
    fn hashmap_repeated_keys() {
        let m = hashmap![1 => "hello", 1 => "world", 2 => "!"];
        assert_eq!(m.len(), 2);
        assert_eq!(m.get(&1), Some(&"world"));
        assert_eq!(m.get(&2), Some(&"!"));
    }

    #[test]
    fn num_items_empty() {
        assert_eq!(num_items![], 0);
    }

    #[test]
    fn num_items_small() {
        assert_eq!(num_items![a, "b", 1, 2.0], 4);
    }

    #[test]
    fn num_items_long() {
        let z = num_items![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67
        ];
        assert_eq!(z, 67);
    }
}
