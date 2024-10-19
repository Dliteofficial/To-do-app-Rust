use chrono::{DateTime, Local};
use std::io::{self, Write};

#[derive(Debug)]
struct Item {
    date_and_time: DateTime<Local>,
    item_title: String,
    item_description: String
}

impl Item {
    fn new(title: String, description: String) -> Self{
        Item { 
            date_and_time: Local::now(),
            item_title: title,
            item_description: description
        }
    }

    fn fetch_title (&self) -> &str {
        &self.item_title
    }

    fn fetch_description (&self) -> &str {
        &self.item_description
    }

    fn fetch_date (&self) -> String {
        self.date_and_time.format("%Y-%m-%d %H:%M").to_string()
    }
}

#[derive(Debug)]
struct ToDo {
    items: Vec<Item>
}

impl ToDo {
    fn new() -> Self {
        ToDo { items: vec![] }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn remove_item(&mut self, index: usize) -> Option<Item>{
        if self.items.len() > index {
            Some(self.items.remove(index))
        }else {
            None
        }
    }

    fn fetch_item(&self, index: usize) -> Option<&Item> {
        self.items.get(index)
    }

    fn list_items(&self) {
        self.items.iter().for_each(|item|println!("{} - {}", item.fetch_date(), item.fetch_title()));
    }
}

//function to read input from the user
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); //ensures that the prompt is shown
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    input.trim().to_string()
}

fn main() {
    let mut todo_list = ToDo::new();

    loop {
        println!("\n-- To-Do App --\n");
        println!("1. Add Item");
        println!("2. Remove Item");
        println!("3. Fetch Item");
        println!("4. List Item");
        println!("5. Quit");

        let choice = read_input("Choose and Option: ");
        match choice.as_str() {
            "1" => {
                let title = read_input("Enter item title: ");
                let description = read_input("Enter item description: ");
                let new_item = Item::new(title, description);
                todo_list.add_item(new_item);
                println!("Item added!");
            },
            "2" => {
                let index = read_input("Enter the index of the item: ").parse().expect("Please enter a valid number");
                match todo_list.remove_item(index) {
                    Some(_) => println!("Item Removed"),
                    None => println!("Invalid Index")
                }
            },
            "3" =>{
                let index = read_input("Enter the index of the item: ").parse().expect("Please enter a valid number");
                match todo_list.fetch_item(index) {
                    Some(value) => println!("{} was created on {} and has the following as description: {}", value.item_title, value.fetch_date(), value.item_description),
                    None => println!("Invalid Index")
                }
            },
            "4" => {
                todo_list.list_items();
            },
            "5" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice. Please try again."),
        }
    }
}