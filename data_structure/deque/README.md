# Hand made Deque in Rust

## Notes

Manipulate Ring Buffer

```plain
empty buffer
h
t
x x x x x

add one element from head
t h
1 x x x x

add one more element from head
t   h
1 2 x x x

add one more element from head
t     h
1 2 3 x x

add one element from tail
      h t
1 2 3 x 4
```

## References

- [雙端佇列](https://rust-algo.club/collections/deque/)
