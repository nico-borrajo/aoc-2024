pub fn run(day: &str, function: fn(&str) -> isize) {
    
    let input_file_name = if let Some("t") = std::env::args().nth(1).as_deref() {
        "test_input.txt"
    } else {
        "input.txt"
    };

    let input_path = format!("inputs/{day}/{input_file_name}");
    let input = std::fs::read_to_string(input_path).unwrap();
    println!("\x1b[1mResult:\x1b[0m\n{}", function(&input));
}


// day modules
