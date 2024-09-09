use std::io;
use crate::types;

pub fn get_todos( database: &mut Vec<types::Todo>){
    for i in 0..database.len(){
         println!("{:#?}",database[i]);
    }
    println!("---------------------------")
}
pub fn create_todo(database: &mut Vec<types::Todo>){
    let mut temp_data = String::from(""); 
    println!("[!] Title: ");
    io::stdin().read_line(&mut temp_data).expect("Error while taking input.");
    let temp_data = temp_data.trim();
    let title:String = temp_data.to_string();
    let new_todo = types::Todo {
        id:database.len() as u32 + 1,
        title
    };
    database.push(new_todo);  
    println!("[+] Todo Create Successfully");  
    println!("---------------------------")
}
pub fn delete_to_by_index(database: &mut Vec<types::Todo>, index:u32){
    database.remove((index - 1)as usize);
    println!("[+] Todo {} deleted sucessfully.",index);
    println!("---------------------------")

}
pub fn clear_all_todos(database: &mut Vec<types::Todo>){
    database.clear();
    println!("[+] All todos deleted.");
    println!("---------------------------")
}

 