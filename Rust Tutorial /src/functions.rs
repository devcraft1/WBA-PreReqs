pub fn run() {
    greeting("Hello", "Dennis");
    let n4 = 30;

    // Binding function values
    let get_sum = add(20, 30);
    println!("Sum {}", get_sum);

    // Closures
    let add_sum = |n1: i32, n2: i32| n1 + n2 + n4;
    println!("C Sum: {}", add_sum(20, 30))
}

fn greeting (greet: &str, name: &str) {
    println!("{} {} Nice to meet you", greet, name,)
}

fn add (n1: i32, n2: i32) -> i32 {
      n1 + n2

    //   So, variables in global scope cannot be accessed  from a function. Wowwwww
}

// Store reusable blocks of code