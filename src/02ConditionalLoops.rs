fn main(){

    // Conditional Loops in rust

    let is_even = true;

    if is_even {
        println!("Number is even");
    } else if !is_even {
        println!("Number is odd");
    } else {
        println!("Number is 0");
    }

    // for loop

    for i in 0..10 {
        println!("i : {}", i);
    }

    // generally there are three things to iterate over : arrays, maps, strings

    let sentence = String::from("My name is ITACHI");
    let first_word = get_first_word(sentence);
    println!("First word : {}", first_word);
}

fn get_first_word(sentence: String) -> String {

    let mut ans = String::from("");

    for char in sentence.chars() {

        ans.push_str(char.to_string().as_str());

        if char == ' ' {
            break;
        }
    }

    return ans;
}