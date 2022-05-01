// functions1.rs
// Make me compile! Execute `rustlings hint functions1` for hints :)

// I AM NOT DONE

fn call_me()->u8 {
    const age:u8 = 21;
    const name:&str = "Victor";

    println!("{} is {} years old.", name, age);
    return age;
}

fn main() {
    call_me();
}
