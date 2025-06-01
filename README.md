# Hello World in Rust with Dynamic Delay

This project is a "Hello, World!" example in Rust, but with a twist: each character of the message is printed to the terminal with a small delay, creating a typing effect. The delay value is dynamically calculated using the system timestamp at runtime.

## How it works

- By default, the program prints the message `Hello, World!`, but you can pass an argument to customize the name:
  ```
  cargo run YourName
  ```
  Example output: `Hello, YourName!`
- Each character is printed one by one, with a delay between them.
- The delay is calculated using the last digits of the current system timestamp in milliseconds, making the wait time different on each execution.

## Code details

- **Macros:**  
  - `print_char_with_delay!`: prints a character to the terminal and waits for a time calculated from the current timestamp.
  - `hello!`: builds the message and prints each character using the macro above.
- **Dynamic delay:**  
  The delay is obtained by taking the last digits of the timestamp in milliseconds (`SystemTime::now().duration_since(UNIX_EPOCH)`), converting them to a number, and using that as the wait time for each character.
- **Arguments:**  
  The program accepts a command-line argument to customize the message.  
  Example:  
  ```
  cargo run Motto
  ```
  Output:  
  ```
  Hello, Motto!
  ```

## How to run

1. Install [Rust](https://www.rust-lang.org/tools/install) if you haven't already.
2. Clone this repository or copy the files to a local folder.
3. In the terminal, navigate to the project folder and run:

```sh
cargo run
```
or, to customize the name:
```sh
cargo run YourName
```

## Project structure

- `src/main.rs`: Main source code.

## License

This project is free for study and learning purposes.