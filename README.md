# FizzBuzz.rs

### The extensible FizzBuzz implementation I used to learn some of the basics of Rust.

I have barely touched Rust before, but since it seems like a really interesting
and just overall well-designed language, I want to have a proper look into it.

So far, this was mostly a pleasure to write, there were only a few cases
when I encountered something that I am really not familiar with. And as this
was meant to be something quick, I just accepted them as black boxes.

For example in the future I will need to look into the actual difference
between clusires and function in rust, or what `dyn Fn() -> T` actually is.
especially compared to `fn() -> T` and `Fn() -> T`. And why closures need to
be `dyn Fn` not just `fn`. Not even sure when `dyn` is needed.

And because of that I will need to look into what exactly `dyn` means and
how it is different from `impl`.

# How to Run

You need Rust installed on your system. After that you can simply do:
```sh
cargo run
```

