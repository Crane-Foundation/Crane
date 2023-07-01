# Welcome to Crane!
![Crane Banner](pictures/Crane4%20(2)%20(4).jpg)  
Welcome to the official repo for the Crane language
[Crane Community](https://discord.gg/mKPdeUnCmG)


## Who are we?
The Crane Foundation is a group of people who believe in making a language that is both simple and efficient in design.

## Why Crane?
Crane is a basic language designed to make your codebase readable while still maintaining the power you may get from C or Rust

## How does it work?
Crane compiled to [CBVM] (https://github.com/sam-buckley/cbvm) bytecode which means it's less portable than an exe but allows cross compilation, cbvm is very fast and efficient and allows easy compilation through its API

## Contributing
We are always open to contributions, so just open up a thread and submit a pull request. If we like it and it works we'll merge!

## Docs
The docs are a WIP and this will be updated once available

# Syntax
Crane syntax is C styled with aspects of Rust, Python and ruby

```rb
def main() -> Void {
  std.println("Hello world!")
}
```
(No semi colons)

```py
def fibo(0) -> Int {0}
def fibo(1) -> Int {1}
def fibo(n: Int) -> Int {
    fibo(n - 1) + fibo(n-2)
}
```
inspired by haskell

## Contributors

<a href = "https://github.com/Crane-Foundation/Crane/graphs/contributors">
    <img src = "https://contrib.rocks/image?repo=Crane-Foundation/Crane"/>
</a>
