//This library returns a random book from the below book list.
use rand::Rng;

//a constant array to store the book list
pub const BOOKS: [&str; 10] = [
    "Cien años de soledad (Gabriel Garcia Marquez, 1967)",
    "Pride and Prejudice (Jane Austin, 1813)",
    "West with the Night (Beryl Markham, 1942)",
    "Harry Potter and the Philosopher's Stone (J.K.Rowling, 1997)",
    "Harry Potter and the Chamber of Secrets (J.K.Rowling, 1998)",
    "Harry Potter and the Prisoner of Azkaban (J.K.Rowling, 1999)",
    "Harry Potter and the Goblet of Fire (J.K.Rowling, 2000)",
    "Harry Potter and the Order of the Phoenix (J.K.Rowling, 2003)",
    "Harry Potter and the Half-Blood Prince (J.K.Rowling, 2005)",
    "Harry Potter and the Deathly Hallows (J.K.Rowling, 2007)",
];
//a function that returns a random book in the list
pub fn random_book() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..BOOKS.len());
    BOOKS[random_index]
}
