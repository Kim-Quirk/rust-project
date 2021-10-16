use std::io;

fn main() {
    //Initiate values
    let mut loop_num: f64 = 0.0;
    let mut goal: f64 = 0.0;
    let mut vec: Vec<f64> = Vec::new();
    let choice: i32;

    loop {
        //If it's the first time through, ask the user how many values they would like to enter.
        //The returned value will be the "goal" number of loops.
        if loop_num == 0.0 {
            goal = user_input(&loop_num);
        }
        //If it is not the first time through, loop to gather the user's input and store in a vector.
        else {
            vec.push(user_input(&loop_num));
        }
        loop_num += 1.0;
        if loop_num >= goal + 1.0 {
            //Continue looping until we reach the user's specified "length" or "goal" for their vector.
            break;
        }
    }

    choice = calculation_choice();
    
    if choice == 1 {
        add_function(&mut vec);
    } else if choice == 2 {
        subtract_function(&mut vec);
    } else if choice == 3 {
        multiply_function(&mut vec);
    } else if choice == 4 {
        divide_function(&mut vec);
    } else {
        println!("Invalid choice.");
    }
}

fn calculation_choice() -> i32 {
    let user: i32;
    loop {
        let mut input = String::new();
        //Select which operation the user would like to implement.
        println!("Please select what operation you would like to perform.");
        println!("1 - Addition (Adds all values.)");
        println!("2 - Subtraction (Negates all values and adds them.)");
        println!("3 - Multiplicaiton (Multiplies all values.)");
        println!("4 - Division (First value is the dividend, all follow are divisors. Will not work with zeros.");
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Convert to an i32.
        // If successful, bind to a new variable named input.
        // If failed, restart the loop.    
        let input: i32 = match input.trim().parse::<i32>() {
            Ok(parsed_input) => parsed_input,
            Err(_) => continue,
        };
        user = input as i32;
        return user; //return the input
    }
}
 
//A function to collect and check if the uer's input is valid.
fn user_input(loop_num: &f64) -> f64 {
    let user: f64;
    loop {
        let mut input = String::new();
        if *loop_num as f64 == 0.0 { //If it's the first time through, ask the user how many values they would like to enter.
            println!("How many values do you want to enter?");
            io::stdin().read_line(&mut input)
            .expect("Failed to read input.");
            // Convert to an i32.
            // If successful, bind to a new variable named input.
            // If failed, restart the loop.
            let input: i32 = match input.trim().parse::<i32>() {
                Ok(parsed_input) => parsed_input,
                Err(_) => continue,
            };
            if input as i32 == 0 { //Check to make sure the user didn't say they wanted to enter "zero" values. 
                //Restart the calculator in this case.
                println!("Enter a value greater than zero to use this calculator.");
                main();
            }
        } else { //If we reach here, the calculator is looping through and collecting the user's input in a vector to perform calculation.
            // Reads the input from STDIN and places it in the String named input.
            println!("Enter a value #{}:", loop_num);
            io::stdin().read_line(&mut input)
            .expect("Failed to read input.");
        }
        // Convert to an f64.
        // If successful, bind to a new variable named input.
        // If failed, restart the loop.
        let input: f64 = match input.trim().parse::<f64>() {
            Ok(parsed_input) => parsed_input,
            Err(_) => continue,
        };
        user = input as f64;
        return user;
    }
}

//A function used to add all the values in the vector
fn add_function(vec: &mut Vec<f64>) {
    //Checks if only one value was entered.
    if vec.len() == 1 {
        println!("Result of adding all values: {}", vec[0]); //If so, it displays that value.
        return;
    }
    //Initilizes values
    let mut z: f64 = 0.0;
    let mut i = 0;
    loop {
        //Checks if the vector has a length is equal to the index.
        if vec.len() == i {
            break; //If so, we're done adding and exit the loop.
        } else
        {
            //Sums up all values and increiments i
            z += vec[i] as f64;
            i += 1;
        }       
    }
    //Prints the result
    println!("Result of adding all values: {}", z);
}

//A function used to subtract all the values in the vector
fn subtract_function(vec: &mut Vec<f64>) {
    //Checks if only one value was entered.
    if vec.len() == 1 {
        println!("Result of subtracting all values: {}", vec[0]); //If so, it displays that value.
        return;
    }
    //Initilizes values
    let mut z: f64 = 0.0;
    let mut i = 0;
    loop {
        //Checks if the vector has a length is equal to the index.
        if vec.len() == i {
            break; //If so, we're done adding and exit the loop.
        } else
        {
            //Subtract all values and incriments i
            //Basically negative addition. All values inverted then added up.
            z += -vec[i] as f64;
            i += 1;
        }       
    }
    //Prints the result
    println!("Result of subtracting all values: {}", z);
}

//A function used to multiply all the values in the vector
fn multiply_function(vec: &mut Vec<f64>) {
    let inf = f64::INFINITY; //Initilize infinity.
    if vec.len() == 1 {
        //Checks if only one value was entered.
        println!("Result of multiplying all values: {}", vec[0]); //If so, it displays that value.
        return;
    }
    //Rust's largest floating point value is an f64. This warning let's users know there may be floating
    //point errors, especially as the numbers get bigger.
    println!("WARNING: Multiplication may suffer from rounding point errors with larger or more values.");

    let mut z: f64 = 0.0; //initlize values
    let mut i = 0;
    loop {
         //Checks if the vector has a length is equal to the index.
        if vec.len() == i {
            break; //If so, we're done adding and exit the loop.
        } else
        { //Loop to do calculations...
            if i == 0 {
                //Multiplies the first and second value and stores it.
                z = vec[i] as f64 * vec[i+1];
                i += 1;
            } else {
                //If we've looped once, multiply the result with indexed value.
                z = z * vec[i] as f64;
            }
            i += 1;
        }       
    }
    if z == -inf || z == inf {
        //If we got infinity something went wrong... Display error.
        println!("The result of multiplying all values resulted in an error.");
        return;
    }
    println!("Result of multiplying all values: {}", z); //Display result.
}

//A function used to divide all the values in the vector
fn divide_function(vec: &mut Vec<f64>) {
    let inf = f64::INFINITY; //initlize infity
    if vec.len() == 1 {
        //Checks if only one value was entered.
        println!("Result of dividing all values: {}", vec[0]); //If so, it displays that value.
        return;
    }
    //Rust's largest floating point value is an f64. This warning let's users know there may be floating
    //point errors, especially as the numbers get bigger.
    println!("WARNING: Division may suffer from rounding point errors with larger or more values.");
    
    let mut z: f64 = 0.0; //initlize values
    let mut i = 0;
    loop {
        //Checks if the vector has a length is equal to the index.
        if vec.len() == i {
            break; //If so, we're done adding and exit the loop.
        } else
        {
            if vec[i] == 0.0 && i != 0 {
                //Checks if value is a zero. The first value can be a zero though, 
                // so only applied when we are not on the first loop.
                println!("Cannot divide by zero. Calculation aborted.");
                return;
            }
            if i == 0 {
                if vec[i + 1] == 0.0 {
                    //Checks if denominator is zero for setting up Z.
                    println!("Cannot divide by zero. Calculation aborted.");
                    return;
                }
                //Sets up z for the first loop.
                z = vec[i] as f64 / vec[i+1];
                i += 1;                
            } else {
                //Divides all values.
                z = z / vec[i] as f64;
            }
            i += 1;
        }       
    }
    if z == -inf || z == inf {
        //If we got infinity something went wrong... Display error.
        println!("Sorry, but dividing all values resulted in an error.");
        return;
    }
    println!("Result of dividing all values: {}", z); //Display result.
}