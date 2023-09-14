const MY_VALUE: u16 = 55;

fn main() {
    /*
     * variables
     */
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    println!("My const is {MY_VALUE}");

    let x = 13;
    {
        // scope
        let x = 15;
        println!("The value of x in the scope is {x}");
    }
    println!("The value of x outside the scope is {x}");
    println!();

    /*
     * data types
     */
    // tuples
    let tup: (i32, u32, char) = (-1354, 123, 'A');
    println!("A tuple {tup:?}");
    let (x, y ,z) = tup;
    println!("Unpacked {x} {y} {z}");
    let tup0 = tup.0;
    let tup1 = tup.1;
    let tup2 = tup.2;
    println!("elements are {tup0} {tup1} {tup2}");

    // arrays
    let a: [u8; 3] = [1, 2, 3];
    println!("array {a:?}");
    println!();

    // functions and expressions
    my_runc(8);
    let x = func_with_return();
    println!("func returned {x}");

    // expressions are inline functions which are evaluated immediately.
    // return keyword is not needed you can just mention the
    // variable without a ;
    let y = {
        let x = 3;
        x + 1
    };
    println!("expression returned {y:?}");

    /*
     * control flows
     */
    let x = 5;
    if x < 6 {
        println!("x is bigger than 6");
    } else {
        println!("x is smaller than 6");
    }
    // conditions MUST be boolean
    // if x is not valid because x is an int
    let number = if x > 3 { 1 } else { 0 };
    println!("inline condition assigned {number}");

    /*
     * Loops
     */
    let mut counter = 0;
    // you can assign loop returns
    let result = loop {
        counter += 1;

        if counter == 10 {
            // returns the value
            break counter * 2;
        }
    };
    println!("loop result is {result}");

    // loops can have labels so you can break a specific loop
    'my_label: loop {
        loop {
            // this breaks the outside loop and with it the inside
            break 'my_label;
        }
    }

    // for
    let a = [1,5,6,8,7];
    for item in a {
        println!("a is {item}");
    }

    // you can make lists and reverse them
    // (1..4) = [1,2,3]
    for number in (1..4).rev(){
        print!("{number}, ");
    }
    println!();
}

fn my_runc(x: i32) {
    println!("I am a func with param {x}");
}

fn func_with_return() -> u32 {
    5 // same as: return 5;
}
