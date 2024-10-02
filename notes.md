# Logs

### 2023-11-22

When I first installed `rustc` and the `rust-analyzer` plugin in VS Code, the plugin was not working. I found [this thread](https://github.com/rust-lang/rust-analyzer/issues/11558#issuecomment-1054802255). I solved the problem following the second option. But also remember that I was installing on a raspberry pi which might use an older OS.

The [difference](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#statements-and-expressions) between statements and expressions is interesting. Statements do not return values while expressions evaluate to a resultant value. Other languages seem to ignore this discussion altogether.

Variable shadowing is very useful, compared to needing to create variables like `x_str` and `x` in C++. This feature is similar to a dynamically typed languge like Python.
