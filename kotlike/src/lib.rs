extern crate proc_macro;
extern crate quote;
extern crate syn;
use regex::Regex; // 1.1.8

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
/**
 * Kotlin question mark style
 *
 * With it you can easily pick out the value wrapped in numberless `Option<T>` and `Result<T,Err>`
 * Furthermore, it won't break down your control flow. You can continue your work even you got a `None` or `Err` (All the unexpected value will turn to `None`)
 * It means:
 * ```
 * let value: Option<TypeYourWant> = wrapped_value?.something_return_option()?.something_return_result()?.value;
 * ```
 * would works fine! Just as Kotlin's style. And you don't need to worry about what `Err` it would throw!
 * ```
 * #[kotlike]
 * fn main() {
 *     let a = "Hello".to_string();
 *     let c: Option<()> = File::create("test.txt")?.write_all(a.as_bytes())?.clone();
 *     let mut b: String = String::new();
 *     let len: Option<usize> = File::open("test.txt")?.read_to_string(&mut b)?.clone();
 *     println!("Hello, {:?}({:?})!", b,len);
 * }
 * ```
 * Expand macro like:
 * ```
 * fn main() {
 *   let a = "Hello".to_string();
 *   let c: Option<()> = File::create("test.txt").map_or(None, |mut v| v.write_all(a.as_bytes()).map_or(None, |mut v| Some(v.clone())));
 *   let mut b: String = String::new();
 *   let len: Option<usize> = File::open("test.txt").map_or(None, |mut v| v.read_to_string(&mut b).map_or(None, |mut v| Some(v.clone())));
 *     println!("Hello, {:?}({:?})!", b,len);
 * }
 * ```
 * Above example is just showing how it works. Don't focus too much on what stupid code does.
 */
#[proc_macro_attribute]
pub fn kotlike(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_clone = item.clone();
    let _func = parse_macro_input!(item as ItemFn); // Cheat compiler
    let raw = item_clone.to_string();
    let seperator = Regex::new(r";[\s]+").expect("Invalid regex");
    let new_item = seperator
        .split(&raw)
        .map(|line| {
            let line = line.to_string();
            let split = line.split("?.");
            let split_size = split.clone().into_iter().count();
            if split_size <= 1 {
                line
            } else {
                split.enumerate().fold(String::new(), |mut acc, (j, item)| {
                    let s = item.to_string();
                    let tmp = if j == split_size - 1 {
                        let postfix = if s.ends_with(";") { ";" } else { "" };
                        let s_trim = s.trim_end_matches(|c: char| c == ';');
                        let repeat = ")".repeat(split_size - 1);
                        format!(".map_or(None,|mut v|Some(v.{s_trim}){repeat}{postfix}")
                    } else {
                        format!(".map_or(None,|mut v|v.{s}")
                    };
                    if acc.is_empty() {
                        acc.push_str(&s);
                    } else {
                        acc.push_str(&tmp);
                    }
                    acc
                })
            }
        })
        .collect::<Vec<String>>()
        .join(";\n");
    // println!("{}", new_item);
    new_item.parse().unwrap()
}
