use std::io;
mod helper;
mod types;

 fn main() {
    

    let mut database: Vec<types::Todo> = Vec::new();
    loop {
        println!("\nTODO");
        println!("");
        println!("Select the option: ");
        println!("1.Get Todos");
        println!("2.Create Todos");
        println!("3.Delete Todo");
        println!("4.Clear All Todos");
     
        println!("[!] Option: ");
        let mut temp_data = String::new();
        io::stdin()
            .read_line(&mut temp_data).expect("Failed to readline.");
        let temp_data = temp_data.trim();
    
             
        if temp_data == "1"{     
            helper::get_todos(&mut database);
        }
        else if temp_data == "2" {
            helper::create_todo(&mut database);
            
        }
        else if temp_data == "3" {
            let mut index:String = String::new();
            println!("[!] Please Enter The index of todo: ");
            io::stdin().read_line(&mut index).expect("Error while taking index.");
            // Attempt to convert the input string to u32
             match index.trim().parse::<u32>() {
                Ok(number) =>  helper::delete_to_by_index(&mut database, number), 
                Err(_) => println!("Invalid input! Please enter a valid u32 number."),  
            }

           
            
        }
        else if temp_data == "4"{
            helper::clear_all_todos(&mut database)
        } 
        else {
            println!("Invalid Option.");
        }
        
    }
   



    // println!("{}",temp_data);


}
