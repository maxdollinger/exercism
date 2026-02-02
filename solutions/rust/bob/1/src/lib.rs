pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    let has_letters = trimmed.chars().any(|c| c.is_alphabetic());
    let is_yelling = trimmed
        .chars()
        .all(|c| !c.is_alphabetic() || c.is_uppercase())
        && has_letters;
    let is_question = trimmed.ends_with("?");

    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }

    if is_question && is_yelling {
        return "Calm down, I know what I'm doing!";
    }

    if is_question {
        return "Sure.";
    }

    if is_yelling {
        return "Whoa, chill out!";
    }

    "Whatever."
}
