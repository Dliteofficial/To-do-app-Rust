use chrono::{DateTime, Local};

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
}

fn main() {

    let new_item = Item::new(String::from("SomeTitle"), String::from("SomeDescription"));

    let mut todo_list = ToDo::new();
    todo_list.add_item(new_item);

    match todo_list.fetch_item(0) {
        Some(value) => {
            println!("Title: {:#?}", value.fetch_title());
            println!("Date: {:#?}", value.fetch_date());
            println!("Description: {:#?}", value.fetch_description());
        },
        None => println!("Index Out of Bounds")
    }
}