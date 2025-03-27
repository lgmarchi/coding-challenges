use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

pub fn verify_chars_ocurrence_on_a_string_are_equal(s: String) -> bool {
    let vec_chars = s.chars().collect::<Vec<char>>();
    let mut count_vec = Vec::new();

    for i in 0..vec_chars.iter().len() {
        let mut count_temp = 0;
        for letter in vec_chars.iter() {
            if letter == &vec_chars[i] {
                count_temp += 1;
            }
        }
        if i != 0 {
            for item in count_vec.iter() {
                if item != &count_temp {
                    return false;
                }
            }
        }

        count_vec.push(count_temp);
    }

    true
}

pub fn verify_chars_ocurrence_on_a_string_are_equal_with_hashmap(
    s: String,
) -> bool {
    let vec_chars = s.chars().collect::<Vec<char>>();
    let mut hashmap_chars: HashMap<char, u32> = HashMap::new();
    for char in vec_chars {
        hashmap_chars
            .entry(char)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let values: HashSet<u32> = hashmap_chars.values().cloned().collect();

    values.len() == 1
}
