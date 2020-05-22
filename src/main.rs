fn main() {
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    println!("{0}, this is {1}. {1}, this is {0}", "Bruno", "Witty");

    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over"
    );

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);


    println!("{number:>width$}", number=1, width=6);

    println!("{number:>0width$}", number=1, width=6);

    // println!("My name is {0}, {1}, {0}", "Bond");

    struct Structure(i32);

}