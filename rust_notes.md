## Introduction

* A **trait** in Rust is a group of methods that are defined for a particular type. 
  * Traits are an abstract definition of shared behavior amongst different types. 
  * So, in a way, traits are to Rust what *interfaces* are to Java or *abstract classes* are to C++.
  * A trait method is able to access other methods within that trait.

* `Vec::with_capacity()` function performs the same task as `Vec::new` but with an important difference. 
  * It preallocates space in the vector.
  * This is very useful and efficient if you know the size of elements you will be storing in the vector.
  * Doing this kind of allocation is slightly more efficient than using `Vec::new`, which resizes itself as elements are inserted.

## Common Collection 

### Storing Lists of Values with Vectors 
* Vectors allows you to store more than one value in a single data structure that puts all the values next to each other in the memory.
* Vectors can only store values on one type.
 ```rust
  let v: Vec<i32> = Vec::new(); 
``` 
* You can also create vector with initial values and Rust will infer the type of value you want to store.
 ```rust
  let v = vec![1,2,3]; 
```

 
* Because we have given initial `i32` values, Rust can infer that the type of `v` is `Vec<i32>`, and the type annotation isn't necessary.

* When the vector gets dropped, all of its contents are also dropped, meaning those integers it holds will be cleaned up

```rust
let v = vec![1, 2, 3, 4 ,5];

let third: &i32 = &v[2];
println!("The third element is {}", third),

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

## Keypoints

* You can't have mutable and immutable references in the same scope.
For example,
```rust 
let mut v = vec![1, 2, 3, 4, 5];

let firt = &v[0];

v.push(6);

println!("The first element is: {}", first);
```

Compiling the above code will result in an error:

```
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 | 
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 | 
8 |     println!("The first element is: {}", first);
  |                                          ----- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `collections` due to previous error
```
The reason for the above error is because of the way `vectors` work. Vectors put the values next to each other in the memory, adding a new element onto the end of vector might require allocating new memory and copying the old elements to the new space.

* To change the3 vallue that the mutable reference refers to, we have to ue the `*` dereference operator to get to the value in `i` before we can use the `+=` operator.
```rust
let mut v = vec![1,2,3,4];
for i in &mut v {
    *i += 50;
}
```

* In the context of collections, the `strings` are implemented as collection of `bytes`, plus some methods to provide useful functionality when those bytes are interpreted as text.

* Rust only has one type of String type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`
* In Rust, both `String` and string slices `&str` are used. Both String and string slices are UTF-8 encoded
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("World !");
let s3 = s1 + &s2;
```
Now, `s1` cannot be used.

## Scoping Rules 
### RAII (Resource Acquisition Is Initialization)
* Whenever an object goes out of scope, its destructor is called and its owned resources are freed.
* This behavior shields against *resource leak bugs*, so one never have to worry about memory leaks.


## Creating a Hash Map
* One way to create an empty HashMap is using portion of the standard library. Of our three common collections, this one is the least often used, so it's not included in the features brought into scope automatically in the prelude.
* Just like vectors, hash map store their data on the heap. 
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert[String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
```
* Rust can infer types that the hashmap contains based on the types of data in the vectors.
* Once you define an Hashmap, the variables defined before will not be the owner anymore. The ownership will move to the hashmap now.
* The `or_insert` method on the `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if the key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.
* The `or_insert` method returns a mutable reference (`&mut v`) to the value specified for the key.
* By default `HashMap` uses a hashing function called *SipHash* that can provide resistance to Denial of Service (DDOS) attacks involving hash tables. This is not the fastest hashing algorithm, but the tradeoff for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hasher is too slow for your purpose, you can switch to another function, by specifying your own hasher.


## Error Handling
* Rust has `panic!` macro to handle exceptions. 
  * When the panic macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.
  * We'll commonly invoke a panic when a bug of some sort has been detected and we don't know how to handle the problem at the time we're writing the program.
  * *Unwinding* means Ruwst walks back up the stack and cleans up the data from each function it encounters. Unwinding and cleaning up is a lot of work, therefore Rust also allows you to immediately *abort* the program without cleaning up. Memory will then be cleaned up by the operating system.
  ```rust
  [profile.release]
  panic = 'abort'
  ```

* `RUST_BACKTRACE` can be set at the environment when doing `cargo run`. A *backtrace* is a list of all the functions that have been called to get this point. Backtraces in Rust work as they do in other languages.

### Recoverable Errors with Result 
* Most errors are not necessarily to abort the program, but instead they have to be handled differently. For example, opening a file and that operation fails because that file doens't exists. So instead of terminating the program, we need to create the file.
* The return type of a `File` is a `Result` type.
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
  }
```
* `T` is for Ok and `E` is for Error.

```rust
use std::fs::File;

fn main() {
    let f = File::open("Hello.txt");

    let f = match f {
        Ok(file) => file, 
        Err(error) => panic!("Problem opening the file: {:?}", error),
      };
  }
```
In the above example, `Result` enum and its variant have already been brought into the scope by the prelude, so we don't need to specify `Result::` before the `Ok` and `Err` variants in the match arm.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
             other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```
Alternatively, to avoid using match at every panic stage, we can use something like this

```rust 
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

### To panic! or not panic! 
* How do we decide when one should call `panic!` and when should one return `Result` 
* When code panics, there is no way to recover.
* Call `panic!` for any error situation, whether there's a possible way to recover or not, but then you're making the decision that a situation is unrecoverable on behalf of the calling code.
* When you return `Result`, you're giving the calling code the options to choose whether it wants to attempt to recover in a way that's appropirate for its situation, or it could decide that an `Err` value in this case is unrecoverable, so it can call `panic!`.
* You can choose not to call `panic!` in case where you know there is a logic that the compiler wouldn't understand, so the `Result` will have an `Ok` value.
* Now you will have a `Result` value that you need to handle; whatever operation you're calling still has the possibility of failing in general.
* For example,
```rust 
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
  .parse()
  .expect("Hardcoded IP address should be valid");
```
In this example, in the case when a user enters a IP address, that would not parse, then instead of calling `panic!`, the code would still return a `Result`.

### Guidelines for Error Handling 
* It's advisable to have your code panic when it's possible that your code could end up in a bad state.
* A *bad state* is when some assumption, guarantee, contract or invariant has been broken, such as when invalid or contradictory values, or missing values are passed to your code.
* Return an error, whenever someone's calling your code and passes in values that doesn't makes sense.
* `panic!` is often appropriate whenver you are calling external code that is out of your control and it returns an invalid state that you have no way of fixing.
* However when failure is expected, it's more appropriate to return a `Result` than to maken a `panic!` call.
* Error out on invalid data as attempting to operate on invalid data can expose you code to vulnerability. This is the main reason the standard library will call `panic!` if you attempt an out-of-bounds memory access.

### Creating Custom Types for Validation 
* Lets say you want to take a guess within range 1 to 100. To do this, you would need to parse the guess as `i32` instead of only `u32` to allow potentially negative numbers, and then add a check for the numbers being in range, like so:
```rust 
loop {
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue, 
      };

    if guess < 1 || guess > 100 {
        // .... snip
      }
  }
```

* The above example is not an ideal solution, because if it was critical that the program only operated on values between 1 and 100, and it had many functions with this requirement, having a check like this in every function would be tedious (and might impact performance).
* Instead, we can make a new type and put the validations in a function to create an instance of the type rather than repeating the validations everywhere - that way its safe for funtions to use the new type in their signatures and confidently use the values they receive.
```rust 
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

## Generic Types, Traits and Lifetimes 

### Validating References with Lifetimes 
* Lifetimes ensure that references are valid as long as we need them to be.
* Every Reference in Rust have a *lifetime*, which is the scope forn which that reference is valid. Most of the time lifetimes are implicit and inferrerd, just like most of the time, types are inferred.
* We must only annotate types when multiple types are possible. In a similar way, we must only annotate lifetimes when the lifetimes of the references could be related in a few different ways.
* Annotating lifetimes is something that is unique to Rust and most programming languages don't even have this concept.
* The main aim of lifetimes is to prevent *dangling* references, which canuse a program to reference data other that the data it's intended to reference.
* Example,
```rust 
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
  }
```
* The example above has an outer scope and an inner scope.
  * The outer reference declares a variable named `r` with no initial value, and the inner scope declares a variable named `x` with the initial value of 5.
  * In the inner scope we attempt to set the value of `r` as a reference to `x`. Then the inner scope ends and we attempt to print the value in `r`. This code will not compile because the value `r` is referring to has gone out of scope before we try to use it.
  * `x` will be out of scope as soon as inner scope ends. But the variable `r` is still valid for the outer scope. If Rust allowed this code to work, `r` would be referencing memory that was deallocated when `x` went out of scope, and anything we tried to do with `r` wouldn't work correctly.
* Rust compiler uses something called *Borrow Checker*, to determine if the code above is invalid.
* A reference lifetime 'static indicates that the data pointed to by the reference lives for the entire lifetime of the running program. It can still be coereced to a shorter lifetime.

## Concurrency 
* *Experience inculcates healthy skepticism, if not outright cynicism, towards all multithreaded code.*
* Handling concurrent programming safely and efficiently is another of Rust's major goals.
* *Concurrent Programming* is where different parts of a program execute independently 
* *Parallel Programming* is where different parts of a program execute at the same time, are becoming increasingly important as more computers take advantage of their multiple processors.
* Rust's *Ownership* and *Type Checking* are powerful set of tools that are leveraged to help manage Memory Safety and Concurrency problems.
  * Many concurrency errors are compile-time errors in Rust rather than run-time. Hence saving time to reproduce the exact problem, Rust would not allow you to write incorrect code in the first place.
  * You can fix your code while you're working on it rather than potentially after it has been shipped to production.
* *Message Passing concurrency*, where channels send messages between threads.
* *Shared-state* concurrency, where multiple threads have access to some piece of data.

### Improving Throughput with Thread Pool
* A **thread pool** is a group of spawned threads that are waiting and ready to handle a task
* A `thread` is created by calling `std::thread::spawn`. It's a fucntion that takes a **closure**.
  * The **closure** contains the code that will be ran in the thread.
  * The moment that thread is created, it is "detached" from the thread that created it. That means it is totally independent and can outlive the thread that created it (unless the creator is the main thread, if that stops, everything stops).
  * "Outliving the parent thread" means everything passed to the closure must remains valid for the entire program (meaning it has a `'static` lifetime). This ensures that everything in the thread remains valid even when the thread that spawned no longer exists.
  * In practice, this means that you want the closure to take ownership of every variable it uses. This is done by `move` keyword in front of the parameter list of the closure.
* A call to `std::thread::spawn` returns a `JoinHandle`. That handle has a `join` method that blocks the current thread, waiting until the spawned thread is closed to continue executing code.
* When the program receives a new task, it assigns one of the threads in the pool to the task, and that thread will process the task. 
* The remaining threads in the pool are available to handle any other tasks that come in while the first thread is processing.
* When the first thread is done processing its task, it’s returned to the pool of idle threads, ready to handle a new task.
* A thread pool allows you to process connections concurrently, increasing the throughput of your server.
* Limit the number of threads in the pool to a small number to protect us from Denial of Service (DoS) attacks.
* A number of worker threads will be waiting in the pool. Requests are then sent to the pool for processing.
* Pool will maintain a Queue of incoming requests. Each of the threacs in the pool will pop off a request from this queue, handle the request and then ask the queue for another request.
* If each thread is responding to a long running request, subsequent requests can still back up in the queue, but we have increased the number of long running requests we can handle before reaching that point.
```
Note: If the operating system can’t create a thread because there aren’t enough 
system resources, thread::spawn will panic. That will cause our whole server to 
panic, even though the creation of some threads might succeed. 
For simplicity’s sake, this behavior is fine, but in a production thread pool implementation, 
you’d likely want to use std::thread::Builder and its spawn method that returns Result instead.
```

### Channels 
* A *channel* is a one-way conduit for sending values from on thread to another. In other word, it's a thread safe queue.
* A popular method to ensure safe concurrency is message passing. Multiple threads communicate by sending each other messages containing data.
* They are something like Unix pipes: one end is for sending data. and the other end is for receiving. The two ends are typically owned by two different threads.
* Unix pipes are for sending *bytes*, channels are for sending Rust values.
* `sender.send(item)` puts a single value in the channel; `receiver.recv()` removes one.
  * Ownership is transferred from the sending thread to the receiving thread.
  * If the channel is empty, `receiver.recv()` blocks until a value is sent.
* Channels are simple way for threads to work together without using *locks* or *shared memory*.
* The channel implementation that Rust provides is multiple producer, single consumer
  * This means that we cannot just clone the consuming end of the channel.
  * As we don't want to send messages multiple times to multiple consumers, as this could lead to discrepancy in the ownership of values. So we want one list of messages with multiple workers such that each message gets processed only once.
* Additionally taking a job off the queue involves mutating the `receiver`, so the threads need a safe way to share and modify `receiver`; otherwise, we might get race conditions.
  * This is done by using *Thread safe smart pointers*, to share ownership across multiple threads and allow the threads to mutate the value, we need to use `Arc<Mutex<T>>`.
  * The `Arc` type will let multiple workers own the receiver, and `Mutex` will ensure that only one worker gets a job from the receiver at a time.
  * For each new worker, we clone the `Arc` to bump the reference count so the workers can share ownership of the worker.
