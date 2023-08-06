## Learning Rust
- `/src/section_<number>_<order>_name.rs`, the separate modules relevant to the study topics, can be imported and run in `main.rs`

## adding new package

goto `Cargo.toml`

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# here below you add the packages:
rand = "0.8.5"
```

###  Flow control
- IF statements
if <boolean expression> {} else if <boolean expression> {} else
the if statement returns a value which is within the curly braces

```rs
let name:&str = if name == 5 {"five"}else if name == 4 {"four"} else {"other"};
```

it should not have a `;` within the braces, BUT it should have a `;` at the end of statement

- loop

`loop` is an unconditional loop, that can exit with `break` statement, like

```rs
loop {
  break;
}
```

what if there are many nested loops and we wnat to exit a particular one?

we can tag a loop:

```rs
'bob: loop {
  'john: loop {
        break 'bob;
  }
}
```

to continue the loop, if `continue` is encountered, rust will continue the default loop,
the closest one; if it is `continue 'named;` - the rust will continue the named loop

```rs
'bob: loop {
  'john: loop {
        continue 'bob;
  }
}
```

- while loop

`while <bool condition>{}` loop repeats while the condition is true;

```rs
while isTrue() {
  // repeat something...
}
```

it is actually, synthactic sugar around, the same as:


```rs
// while loop:
loop {
  if !isTrue() {break;}
  // do stuff...
}

// or do{} while:
loop {
  // do stuff...
  if !isTrue() {break;}
}
```

- for loop

```rs
for num in [1, 2, 3].iter() {
  print!("{} / ", num);
}
// for loop -  can also destructure
let my_nums = [(1, 2), (3, 4), (5, 6)];

for (num_1, num_2) in my_nums.iter() {
  print!("{} / {} / ", num_1, num_2);
}

// or use of ranges like from 0 to 10
for num in 0..10 {
  print!("{}, ", num);
}
```


### Strings
there are at leas 6 types of strings in rust,
BUT we are going to wrok mostly with just 2 types of strings: `&str`(String slice) and `String`

the most important difference is that `&str` type cannot be modified;
```rs
let name: &str = "hi 👋";
let name_converted: String = "hi 👋".to_string();
```

- we cannot get exact letter just by indexing like : my_name[2], because different characters/letters may have different byte length, so no guarantee that a string "onetwothree😃" has 12 letters or characters
What we can use here is `"somevalue".chars().nth(n)`

```rs
  let name = "john💥silver😜";
  let emoji_bang = name.chars().nth(4);
```