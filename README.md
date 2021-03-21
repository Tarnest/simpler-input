A simpler way to receive input from a user.

## **Usage**
```
simpler-input = "0.2.0"
```

There is only one function, so it's easier to implement and remember.

```rust
use simpler-input::input;

fn main() {
	// if you decide to get user input with no text printed before, you must pass None as an arg
	let a = input(None);
	
	// if you want text printed directly before your program asks for input, you can pass a string
	let b = input("Hello there!");
}
```