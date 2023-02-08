enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // ==== match
    // the power comes from the compiler enforcing that all cases are handled
    // At first sight it might seem similar to an if statement
    // but the condition in If statement can only be booleans, where's with match it can be of any type
    // the patterns are evaluated in order

    // ==== Patterns that bind to values
    // When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state.
    // this is essentially binding the variable inside the match arm to the value attached to the enum variant.

    // ==== Matching Option<T>
    //
    fn plus_one(option: Option<i32>) -> Option<i32> {
        match option {
            None => None,
            // the i binds to the value contained in Some value
            Some(i) => Some(i + 1),
        }
    }

    // ===== Matches are exhaustive
    // We will need to match all arms or else it will fail with an error

    // ===== Catch-all pattern and the _ placeholder
    // We can take a default action for other patterns and defined actions for others
    let dice_role = 0;
    match dice_role {
        3 => add_fancy_hate()
        7 => remove_fancy_hate()
        other => move_player(other)
        // if we don't want to use the value
        // _ => reroll()
        // we can also express nothing should happen by using the unit value ()
        // _ => ()
    }
    fn add_fancy_hate() {};
    fn remove_fancy_hate() {};
    fn move_player(num: u8) {};
    fn reroll() {};
    // the match-all pattern meets the requirement that match need to be exhaustive

    // Rust also has a pattern when we want to have a catch-all but don't want to use the value in the catch-all
    // pattern: _ 

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // this is an arm
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        // commas are optional
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
