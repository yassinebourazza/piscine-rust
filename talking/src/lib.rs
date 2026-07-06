pub fn talking(text: &str) -> &str {
    if text.is_empty() {
        return "Just say something!";
    }
    let question = text.ends_with('?');
    let bool1 = text.chars().any(|c| c.is_alphabetic());
    let bool2 = bool1 && text.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());

     if bool2 && question {
        "Quiet, I am thinking!"
    } else if bool2 {
        "There is no need to yell, calm down!"
    } else if question {
        "Sure."
    } else {
        "Interesting"
    }

}