fn old_main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "     ";
    println!("The length of spaces is: {}", spaces.len());
    
}

fn another_function() {
    println!("\nAnother function");
}

fn main() {
   old_main();
   another_function();
   nested_scope();

   let a = multiplier(3, 4);
   println!("my first multipication: {a}");

   conditions_func();

   my_loop();

   my_while();

   my_rev_loop();
}


fn my_rev_loop() {
    for number in (1..6).rev() {
        println!("{number}!");
    }
    println!("\nReversed Loop\n");
}


fn nested_scope() {
   let y = {
	let x = 3;
	x + 1
   };
   println!("\nprint X is equal to: {y}");
}


fn multiplier(x: i32, y: i32) -> i32 {
	x * y
}


fn conditions_func() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}


fn my_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
	println!("counter now: {counter}");
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}\n\n");
}

fn my_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!\n");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1;
    };
}

