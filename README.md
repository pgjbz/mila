# Mila
What is Mila?

Mila as programme language written in Rust, this lang is a reimplementation of [bzr](https://github.com/pgjbz/bzr), but improved, and removed useless type check in let statements, and real immutable `let` stataments and mutabble `var` statements.

This lang supports objects like:

Supports floating points like:

```mila
let floating_pointers = 1047.0;
```

Supports numbers like:

```mila
let numbers = 100;
```

Supports booleans like:

```mila
let is_mila_lang = true;
```

Supports anonymous function like:

```mila
fn calc(function, a, b) {
    ret function(a, b);
}

putsln("Sum = ", calc(fn(a, b) { a + b}, 1, 2))
```

### Important

This project is just to learn how interpreters works


## Plans

- [x] Reimplement bzr
- [ ] Fix code (semicolon after while)
- [ ] Improve Codes
- [ ] Fix parse errors (FeelsBadMan)