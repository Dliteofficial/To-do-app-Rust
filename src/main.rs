//Create To-do
//Update todo with new items
//delete todo items from list
//read todo item

//Items: vector of items
//each item:
//Date & Time todo was made
//title of item
//description/details

use chrono::{DateTime, Local};

struct Item {
    date_and_time: String,
    item_title: String,
    item_description: String
}

impl Item {
    fn new(title: String, description: String) -> Self{
        Item { 
            date_and_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            item_title: title,
            item_description: description
        }
    }
}

struct ToDo {
    items: Vec<Item>
}

impl ToDo {
    fn new() -> Self {
        ToDo { items: vec![] }
    }

    fn addItem(&mut self, item: Item) {
        self.items.push(item);
    }

    fn removeItem(&mut self, index: usize) {
        self.items.remove(index);
    }

    fn fetch_Item(&self, index: usize) -> &Item {
        &self.items[index]
    }
}

fn main() {

}