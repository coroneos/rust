fn main () {
	let w = first_word("Wacky waving inflatable arm-flailing tube man.");
	println!("{}", w);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}