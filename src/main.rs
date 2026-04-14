// GAME overview

// Player starts the game.

// Player is given a word with empty letters.

// Player types in a letter in attempt to guess the word.

// Player is given ten attempts to guess the word.

// For every guess either wrong or right, the remaining guesses are reduced by one.

// To make things harder, If the player guesses an already guessed letter, the remaining guesses are reduced by one.

// If players guesses all the letters correctly before the guesses are exhausted, the player wins else the player loses.

// Program ends, start again from the beginning.

use rand::rng;
use rand::prelude::IndexedRandom;
use std::io;
#[allow(dead_code)]




fn main() {
    struct  PlayerRoot {
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

//fn new(...) -> PlayerRoot
// This is a constructor function
// It returns a PlayerRoot object

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
            PlayerRoot { word: String::from(word), no_of_guesses, available_alphabates, list_of_words_to_guess_from, output_string, max_tries: max, guess, correct_guesses }
        }

        fn generate_random_words(list: &Vec<String>) -> String {
            let mut rng = rng();
            let word = list.choose(&mut rng).unwrap();
            word.to_string()

        }
    }

    // list of words for the game

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

    let guess_chars = vec!['_'];

    // for our simple UI
    let guess_vec: Vec<char> = random_word.clone().chars().collect();
    let output_string_vec = vec!['_'; guess_vec.len()];

    let mut player_one =   PlayerRoot::new(
        &random_word,
        0,
        letters,
        list_of_words,
        output_string_vec,
        10,
        "".to_string(),
        guess_chars,
    );

    println!("Welcome to the hangman game built with rust!, please enter a letter");
    println!(
        "Fill in the blank spaces{:?}, no of guesses made {:?}, no of max tries {:?}",
        player_one.output_string, player_one.no_of_guesses, player_one.max_tries
    );

    loop {
        //Takes in an input
        //Todo Check if input is more than one char

        let mut guess = String::from("");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let altered_guess: char = match guess.trim().chars().next() {
            Some(val) => val,
            _ => {
                println!("NO letter inputted, type a letter!!");
                '0'
            }
        };



        // check if guess is valid

        if !player_one.output_string.contains(&altered_guess) {
            player_one.no_of_guesses += 1;

            if !player_one.word.contains(altered_guess) {
                let guess_score = player_one.max_tries - player_one.no_of_guesses;
                println!(
                    "Wrong guess\nFIll inthe blank spaces{:?} no of guesses remaining {:?}",
                    player_one.output_string, guess_score
                );

            }

            // loop through the word , check it the guess is correct,reduce number of guess by one
            for n in player_one.word.char_indices() {
                if n.1 == altered_guess {
                    let guess_score = player_one.max_tries - player_one.no_of_guesses;

                    player_one.correct_guesses.push(n.1);
                    player_one.output_string[n.0] = n.1;

                    println!(
                        "Fill in the blank spaces {:?} no of guesses remaining {:?}",
                        player_one.output_string, guess_score
                    );
                }

            }
        } else {
            println!("That letter is Taken !!! guess again")
        }

        if player_one.correct_guesses.len() == guess_vec.len() {
        print!("You Win!!");
        break;
    }
    }
    

}
