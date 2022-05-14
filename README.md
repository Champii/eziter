# EzIter
Iterators made easy

## Usage

Drop this line into your `Cargo.toml`

```toml
eziter = { git = "https://github.com/Champii/eziter" }
```

and add a use statement at the top of every module using `EzIter`

```rust
use eziter::*;
```

## What is this ?

This crate contains some conveniance wrappers around rust's iterators and std collections.

Theses are essentially useful in the rare cases where you only want to use a single map or
filter on your iterator without the syntactic burden of unrolling a full `x.iter().map(f).collect();`.
You can instead write a more appeiling `x.map(f);`

Only a handful of methods are actually implemented, more might come in the future:

```rust
    x.map(f);
    x.filter(f);
    x.filter_map(f);
    x.skip_while(f);
    x.take_while(f);
    x.map_while(f);
```


Each of theses come in 3 flavors, here an example with `map`

```rust
    x.into_map(f);
    x.map(f);
    x.map_mut(f);
```

That corresponds to 

```rust
    x.into_iter().map(f).collect();
    x.iter().map(f).collect();
    x.iter_mut().map(f).collect();
```

You can use theses wrappers out of the box with the standard collections

```rust
    HashMap<K, V>
    BTreeMap<K, V>
    HashSet<K>
    BTreeSet<K>
    BinaryHeap<T>
    LinkedList<T>
    Vec<T>
    VecDeque<T>
```

but note that the `*_mut()` variations are not available for 
```rust
    HashSet<K>
    BTreeSet<K>
    BinaryHeap<T>
```

You also gain usage of all `into_*()` wrappers for every implementors of `IntoIterator`

## Example

```rust
use exiter::*;

fn main() {
    let v = vec![1, 2, 3];

    let _res: Vec<_> = v.map(|x| x + 3);
}
```

## Caveats

Please note that theses wrappers don't come for free.

### No chainable calls

First, you will lose the ability to chain theses calls together.
I mean, you *could* chain them, but they would all individually call the `.collect()` method, 
leaving you with an increasing waste of resources as the chain grows.
The rust's Iterators are the way to go in that case, obviously.

### Extra `Box<>` allocation

In order for the implementation to hold, only the `into_*()` wrappers come really free of charge,
the others produced iterators are wrapped around a `Box<dyn Iterator>`. If you don't mind the extra
allocation/deallocation, you are fine.
This might change in the future with when rust will allow to return `impl`s from trait implementation.

