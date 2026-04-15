// Import random number generation utilities for selecting random words
use rand::prelude::IndexedRandom;
use rand::rng;
// Import standard input/output for user interaction
use std::io;
#[allow(dead_code)]

/// Main entry point for the Hangman game application
fn main() {
    struct PlayerRoot {
        word: String,
        no_of_guesses: i8,
        available_alphabates: Vec<char>,
        list_of_words_to_guess_from: Vec<String>,
        output_string: Vec<char>,
        max_tries: i8,
        guess: String,
        correct_guesses: Vec<char>,
    }

    struct PlayerGuesser {
        guess: char,
        tries: i8,
    }

    impl PlayerRoot {
        /// Constructor function to create a new PlayerRoot instance
        /// Returns a PlayerRoot object initialized with provided values
        fn new(
            word: &str,
            no_of_guesses: i8,
            available_alphabates: Vec<char>,
            list_of_words_to_guess_from: Vec<String>,
            output_string: Vec<char>,
            max: i8,
            guess: String,
            correct_guesses: Vec<char>,
        ) -> PlayerRoot {
            PlayerRoot {
                word: String::from(word),
                no_of_guesses,
                available_alphabates,
                list_of_words_to_guess_from,
                output_string,
                max_tries: max,
                guess,
                correct_guesses,
            }
        }

        /// Generates a random word from the provided list
        fn generate_random_words(list: &Vec<String>) -> String {
            let mut rng = rng();                    // Initialize random number generator
            let word = list.choose(&mut rng).unwrap(); // Select random word from list
            word.to_string()                        // Convert to String and return
        }
    }

    // ============ GAME INITIALIZATION ============
    // Define the list of words that players will guess from
    let list_of_words = vec![
        "hunting".to_string(),
        "dizzy".to_string(),
        "while".to_string(),
        "string".to_string(),
        "something".to_string(),
        "notified".to_string(),
    ];

    let random_word = PlayerRoot::generate_random_words(&list_of_words);

    let letters = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    // Initialize list of guessed characters
    let guess_chars = vec!['_'];

    // Convert the random word to a character vector for easier manipulation
    let guess_vec: Vec<char> = random_word.clone().chars().collect();
    // Create initial display with blanks (underscores) for each letter
    let output_string_vec = vec!['_'; guess_vec.len()];

    let mut player_one: PlayerRoot = PlayerRoot::new(
        &random_word,
        0,
        letters,
        list_of_words,
        output_string_vec,
        10,
        "".to_string(),
        guess_chars,
    );

    // Display welcome message and initial game state
    println!("Welcome to the hangman game built with rust!, please enter a letter");
    println!(
        "Fill in the blank spaces{:?}, no of guesses made {:?}, no of max tries {:?}",
        player_one.output_string, player_one.no_of_guesses, player_one.max_tries
    );

    // ============ MAIN GAME LOOP ============
    loop {
        // Get user input for the guessed letter
        // TODO: Validate that input is exactly one character
        let mut guess = String::from("");
        // Read the user's input from standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Convert input string to a single character, or '0' if invalid
        let altered_guess: char = match guess.trim().chars().next() {
            Some(val) => val,
            _ => {
                println!("NO letter inputted, type a letter!!");
                '0'  // Placeholder for invalid input
            }
        };

        // Check if the letter hasn't been guessed before

        if !player_one.output_string.contains(&altered_guess) {
            // Increment the guess counter
            player_one.no_of_guesses += 1;

            // Check if the guessed letter is NOT in the word (incorrect guess)
            if !player_one.word.contains(altered_guess) {
                let guess_score = player_one.max_tries - player_one.no_of_guesses;
                println!(
                    "Wrong guess\nFIll inthe blank spaces{:?} no of guesses remaining {:?}",
                    player_one.output_string, guess_score
                );
            }

            // Search through the word and reveal all matching letters
            for n in player_one.word.char_indices() {
                // Check if current character matches the guessed letter
                if n.1 == altered_guess {
                    // Calculate remaining guesses
                    let guess_score = player_one.max_tries - player_one.no_of_guesses;

                    // Add the correct guess to the list of correct guesses
                    player_one.correct_guesses.push(n.1);
                    // Reveal the guessed letter at its position in output
                    player_one.output_string[n.0] = n.1;

                    println!(
                        "Fill in the blank spaces {:?} no of guesses remaining {:?}",
                        player_one.output_string, guess_score
                    );
                }
            }
        } else {
            // Letter has already been guessed
            println!("That letter is Taken !!! guess again")
        }

        // Check if the player has guessed all letters in the word (win condition)
        if player_one.correct_guesses.len() == guess_vec.len() {
            print!("You Win!!");
            break;  // Exit the game loop
        }
    }
}
