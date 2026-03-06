use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    
    //In case the file isn't created, we use this
    let mut file = File::create(filename).expect("Could not create file");

    //This loop will go through all the information of each book 
    for i in 0..books.len() {
        let book = &books[i];
        
       
        let mut line = String::new();
        line.push_str(&book.title);
        line.push_str(",");
        line.push_str(&book.author);
        line.push_str(",");
        line.push_str(&book.year.to_string());
        line.push_str("\n");

        let bytes = line.as_bytes();
        file.write_all(bytes).expect("Failed to write");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let mut loaded_books = Vec::new();

    //In case something goes wrong with opening the file
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result.expect("Could not read line");
        
        let mut parts = Vec::new();
        for part in line.split(',') {
            parts.push(part.to_string());
        }
        
        //We'll clone the information of the book 
        if parts.len() == 3 {
            let t = parts[0].clone();
            let a = parts[1].clone();
            
            let y_str = &parts[2];
            let y: u16 = y_str.parse().expect("Year was not a number");

            let new_book = Book {
                title: t,
                author: a,
                year: y,
            };
            loaded_books.push(new_book);
        }
    }
    loaded_books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}
