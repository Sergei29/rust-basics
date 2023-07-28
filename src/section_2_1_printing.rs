pub fn printing_example() {
    /*
     * multiline comment
     * multiline comment
     */

    // single line comment

    //  println!(...) macro always adds a new line after the output
    println!("Hello, world!");
    println!("the first value is {}, the second value is {}", 10, 20);
    println!(
        "My first name is {}, and my email address is {}.",
        "Serge", "serge@gmail.com"
    );

    // print!(...) outputs on the same line until it reaches the end of line
    print!("1. this will print on the same line!");
    print!(" 2. and this!");
    print!(" 3. and this!");

    // unless:
    print!(
        "
  This will print
       over the 
          multiple lines!
  "
    );

    // backslash \ char is used to print special characters
    println!("\n\n This is printed after an empty line.");
    println!("\t This will have a tab space at the beginning.");
    println!(
        "Characters: this will print a single quote \', and a double quote \", and a backslash \\ "
    );

    // indexed parameters
    println!(
        "I\'m doing {2} for {1} years, and I {0} it!",
        "like", 5, "programming"
    );

    // named parameters
    println!(
        "{language} is a system programming language, which is cool to {activity} in. ",
        language = "Rust",
        activity = "code"
    );

    // evaluated expression output
    println!("The sum of 25 + 10 = {}", 25 + 10);
}
