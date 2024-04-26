use std::collections::HashMap;

fn main() {
    let perm1 = "AabCcDEFGHIKJLMNOPQRSTUVWXYzxywv".to_string();
    let perm2 = "RAabCcDEFGHIKJLMNOPQSTUVWXYzxywv".to_string();
    let mut perm = perm1.chars().count() == perm2.chars().count();
    if perm {
        let mut string_hash1 = HashMap::new();
        let mut string_hash2 = HashMap::new();
        for c in perm1.chars(){
            string_hash1.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
        }
        println!("{:?}", string_hash1 );
        for c in perm2.chars(){
            string_hash2.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
        }
        println!("{:?}", string_hash2 );
        perm = string_hash1.eq(&string_hash2);
    }
    match perm {
        true => {
            println!("The second string is a permutation of the first");
        },
        false => {
            println!("The strings are not permutations of the other");
        }
    };
}
