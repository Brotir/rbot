/// A custom print macro that enables regular Rust printing in Bot Beats.
///
/// This macro forwards the formatted string to the `rbot::print` function.
/// It behaves similarly to the standard `print!` macro, but integrates with the
/// `rbot` module's printing functionality.
///
/// # Examples
///
/// ```
/// // Prints nothing
/// print!();
///
/// // Prints "Hello, world!" without a trailing newline
/// print!("Hello, world!");
///
/// // Prints "Number: 42" without a trailing newline
/// print!("Number: {}", 42);
/// ```
///
/// # Usage
/// - When called without arguments, it prints nothing.
/// - When called with arguments, it formats the string according to the specified format and prints it.
///
/// # Arguments
/// - `format`: A format string that specifies how the arguments should be formatted.
/// - `args`: The arguments to format according to the format string.
///
/// This macro makes use of the `format!` macro internally to handle string interpolation and formatting.
#[macro_export]
macro_rules! print {
    // Match when there are no arguments
    () => {
        rbot::print();
    };
    // Match when there is one or more arguments
    ($($arg:tt)*) => {
        rbot::print(&format!($($arg)*));
    };
}

/// A custom println macro that enables regular Rust printing in Bot Beats.
///
/// This macro forwards the formatted string to the `rbot::print` function.
/// It behaves similarly to the standard `println!` macro, but integrates with the
/// `rbot` module's printing functionality.
///
/// # Examples
///
/// ```
/// // Prints a newline
/// println!();
///
/// // Prints "Hello, world!" with a newline
/// println!("Hello, world!");
///
/// // Prints "Number: 42" with a newline
/// println!("Number: {}", 42);
/// ```
///
/// # Usage
/// - When called without arguments, it prints a newline.
/// - When called with arguments, it formats the string according to the specified format and prints it with a newline.
///
/// # Arguments
/// - `format`: A format string that specifies how the arguments should be formatted.
/// - `args`: The arguments to format according to the format string.
///
/// This macro makes use of the `format!` macro internally to handle string interpolation and formatting.
#[macro_export]
macro_rules! println {
    // Match when there are no arguments
    () => {
        rbot::print("\n");
    };
    // Match when there is one or more arguments
    ($($arg:tt)*) => {
        rbot::print(&format!("{}\n", format!($($arg)*)));
    };
}
