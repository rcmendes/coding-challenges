/*
For want of a nail the shoe was lost.
For want of a shoe the horse was lost.
For want of a horse the rider was lost.
For want of a rider the message was lost.
For want of a message the battle was lost.
For want of a battle the kingdom was lost.
And all for the want of a nail.
 */
pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::from("");

    if list.len() == 0 {
        return proverb;
    }
    let mut last_word = list[0];
    for word in list.iter().skip(1) {
        let phrase: String = format!("For want of a {} the {} was lost.\n", &last_word, &word);
        proverb.push_str(phrase.as_str());
        last_word = word;
    }

    proverb.push_str(format!("And all for the want of a {}.", list[0]).as_str());

    proverb
}
