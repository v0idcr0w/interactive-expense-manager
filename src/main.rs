use std::io; 
use std::collections::HashMap; 

fn display_menu() {
    println!("***** Expense Manager *****");
    println!("1. View Expenses");
    println!("2. Add Expense");
    println!("3. Remove Expense");
    println!("4. Edit Expense"); 
    println!("[INFO] Press q to quit at any stage"); 
}

fn view_expenses(expenses: &HashMap<String, f64>) {
    if expenses.is_empty() {
        println!("No entries yet"); 
    } else {
        for (key, value) in expenses {
            println!("{} {}", key, value); 
        }
    }
}

fn add_expense(expenses: &mut HashMap<String, f64>) {
    println!("Please enter the expense name or press q to cancel this operation: ");
    let mut name: String = String::new();  
    io::stdin().read_line(&mut name).expect("Failed to read line"); 
    name = name.trim().to_lowercase(); 
    if name == "q" {
        return (); // early return 
    }
    // this loop will run for as long as there is an error parsing the input amount
    loop {
        let mut amount: String = String::new(); 
        println!("Please enter the expense value: ");
        io::stdin().read_line(&mut amount).expect("Failed to read line"); 

        let amount: f64 = match amount.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
            // only executed if there is a valid amount 
        expenses.insert(name, amount); 
        break; 
    }


}

fn remove_expense(expenses: &mut HashMap<String, f64>) {
    loop {
        println!("Please enter the expense name you want to delete or press q to cancel this operation: ");
        let mut name: String = String::new();  
        io::stdin().read_line(&mut name).expect("Failed to read line"); 
        name = name.trim().to_lowercase(); 
        if name == "q" {
            return (); 
        }
        
        let outcome = expenses.remove(&name); 
        match outcome {
            Some(value) => {
                println!("Successfully removed {} with value of {}", name, value);
                return (); 
            }, 
            None => {
                println!("Operation failed, {} possibly does not exist. Please try again.", name);
                continue; 
            }
        }
    }
}

fn edit_expense(expenses: &mut HashMap<String, f64>) {
loop {
    println!("Please enter the expense name you want to edit or press q to cancel this operation: ");
    let mut name: String = String::new();  
    io::stdin().read_line(&mut name).expect("Failed to read line"); 
    name = name.trim().to_lowercase(); 
    if name == "q" {
        return (); // early return from this function 
    }

    // check if the key exists with this name
    if !expenses.contains_key(&name) {
        println!("Entry with name {} not exist. Please try again.", name); 
        continue; 
    } 

    // prompt user for a new value to replace
    loop {
        let mut new_amount: String = String::new(); 
        println!("Please enter the new expense value: ");
        io::stdin().read_line(&mut new_amount).expect("Failed to read line"); 

        let new_amount: f64 = match new_amount.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        // only executed if there is a valid amount 
        expenses.insert(name, new_amount); 
        return ();  
    } // inner loop

} // outer loop

}



fn main() {
    // let mut expenses: HashMap<String, f64> = HashMap::new();
    let mut expenses = HashMap::from([("supermarket purchase".to_owned(), 50.0),
    ("telephone bill".to_owned(), 29.99),
    ("electricity".to_owned(), 32.17)]);

    loop {
        let mut choice: String = String::new(); 
        display_menu(); 

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line"); 

        // parse choice
        choice = choice.trim().to_lowercase(); 

        println!("{}", choice); 

        match choice.as_str() {
            "1" => {
                view_expenses(&expenses); 
            },
            "2" => {
                add_expense(&mut expenses); 
            },
            "3" => {
                remove_expense(&mut expenses); 
            },
            "4" => {
                edit_expense(&mut expenses); 
            },
            "q" => break,
            _ => {
                println!("Invalid command. Please try again.");
                continue; 
            }
        }

    }

}
