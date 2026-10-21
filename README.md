# stui

![](https://github.com/sopho-s/stui/blob/main/example.gif)

stui is a library to easily create TUI that can easily be added to any program in post (or during its creation)

Any stui ui will give you a set and reset string

```rust
print!("{}\n\r", root.toString()); // prints the ui
print!("{}", root.getResetString()); // clears the ui
```

main.rs is a basic example as shown in the gif on the top of this readme but can be easily modified to fix your needs

For large amounts of inputs, you can take advantage of the form object to get results of all inputs within a form when a submission is recorded to the form

stui uses interrupts to get keyboard inputs, but you should regularly poll the keyboard input queue to update to know when to update the interface
