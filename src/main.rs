//importing rust's built-in HashMap into the program.
use std::collections::HashMap;

//defining the main entry point of the program.
fn main() {

    //getting the command-line arguments, specifying the action and item required.
    let action = std::env::args()
        .nth(1)
        .expect("Please provide an action");
    let item = std::env::args().nth(2).expect("Please provide an item");

    //instantiating a new instance of Todo struct(Database) for the program to use.
    let mut todo = Todo::new().expect("Initialisation of db failed");

    //if the action is add, we will insert a new item in the list and save it.
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo Saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }
    //when the specified action is complete, we check if the given item is present in the list.
    else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => {
                match todo.save() {//save the updated list.
                    Ok(_) => println!("Todo Saved"),
                    Err(why) => println!("An error occurred: {}", why),
                }
            }
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}
//implementing methods for Todo Struct(Database).
impl Todo {
    //defining a method to create a new instance of Todo Struct(Database).
    fn new() -> Result<Todo, std::io::Error> {

        //opens and initializes a file in read mode.
        let f = std::fs::OpenOptions::new().write(true).create(true).read(true).open("db.json")?;

        //if the given file contains data then deserialize it otherwise create a new empty HashMap.
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
    
    //method to insert a new item in the list with status active.
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    //method to save the updated map function to a file.
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;

        Ok(())
    }

    //function to mark the item as complete.
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
