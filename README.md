<div style="text-align: center;">
<h1>Kotlike</h1>
A Rust macro to modify the question mark operator's behavior just like Kotlin's
</div>

With it, you can easily pick out the value wrapped in numberless `Option<T>` and `Result<T,Err>`.

Furthermore, it won't break down your control flow. You can continue your work even you got a `None` or `Err` (All the unexpected value will turn to `None`)

It means:
```rust
fn do_something() {
    let value: Option<TypeYourWant> = wrapped_value?.something_return_option()?.something_return_result()?.value;
}
```
would works fine! Just as Kotlin's style.
And you don't need to worry about what `Err` it would throw!

# How it Works
**Usage**
```rust
#[kotlike]
fn main() {
    let a = "Hello".to_string();
    
    let c = File::create("test.txt")?.write_all(a.as_bytes())?.clone();
    
    let mut b: String = String::new();
    
    let len = File::open("test.txt")?.read_to_string(&mut b)?.clone();
    
    println!("Hello, {:?}({:?})!", b,len);
}
```
**Expand the macro would look like**:
```rust
fn main() {
  let a = "Hello".to_string();
    
  let c: Option<()> = File::create("test.txt")
      .map_or(None, |mut v| {
          v.write_all(a.as_bytes())
              .map_or(None, |mut v| Some(v.clone()))
      });
    
  let mut b: String = String::new();
    
  let len: Option<usize> = File::open("test.txt")
      .map_or(None, |mut v|{
          v.read_to_string(&mut b)
              .map_or(None, |mut v| Some(v.clone()))
      });
    println!("Hello, {:?}({:?})!", b,len);
}
```
Above example is just showing how it works. Don't focus too much on what stupid code does.

# LICENSE
[Apache LICENSE 2.0](LICENSE)