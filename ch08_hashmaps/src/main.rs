use std::collections::HashMap;


fn main() {

    // misc01();

    // misc02();

    // misc03();

    // misc04();

    // let result: HashMap<String, i32> = misc05();
    // println!("&result ``{:?}``", &result);
    // for (key, value) in &result {
    //     println!("{}: {}", key, value );
    // }

    // -- straight overwriting of value --
    // misc06();

    // -- insert value if key does not exist --
    // misc07();

    // -- insert value if key does not exist, AND update value based on old value --
    misc08();

}

fn misc08() {
    let text = "hello world wonderful world";
    let mut hmap = HashMap::new();

    for word in text.split_whitespace() {
        let count = hmap.entry(word).or_insert(0);
        // let zz: () = count;  // yields: found `&mut {integer}`
        *count += 1;  // book says count must be dereferenced because the "or_insert" method returns a mutable reference to the value for this key.
    }  // book says that the "count" mutable reference goes out of scope at the end of the for-loop.
    println!("hmap, ``{:?}``", hmap);
}



// fn misc07() {
//     let mut scores = HashMap::new();
//     scores.insert( String::from("Blue"), 10 );

//     scores.entry( String::from("Yellow") ).or_insert(50);
//     scores.entry( String::from("Blue") ).or_insert(50);

//     println!("scores, ``{:?}``", scores);
// }



// fn misc06() {
//     let mut scores = HashMap::new();
//     scores.insert( String::from("Blue"), 10 );
//     scores.insert( String::from("Blue"), 25 );
//     println!("scores, ``{:?}``", scores);
// }



// fn misc05() -> HashMap<String, i32> {

//     let mut scores = HashMap::new();

//     scores.insert( String::from("Blue"), 10 );
//     scores.insert( String::from("Yellow"), 50 );

//     scores

// }



// fn misc04() {

//     let mut scores = HashMap::new();

//     scores.insert( String::from("Blue"), 10 );
//     scores.insert( String::from("Yellow"), 50 );

//     let team_name = String::from("Blue");

//     let score = scores.get( &team_name );
//     println!("score, ``{:?}``", score);  // Some(10)

// }



// fn misc03() {
//     let field_name = String::from("Some key");
//     let field_value = 42;

//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);

//     println!("map, ``{:?}``", map);

//     // println!("field_name, ``{:?}``", field_name); // nope, because the String type ownes the value
//     println!("field_value, ``{:?}``", field_value);  // yes, because the Integer type implements the Copy trait
// }



// fn misc02() {
//     let teams = vec![
//         String::from("Blue"),
//         String::from("Yellow")
//         ];
//     let initial_scores = vec![10, 50];

//     //  -- hmm; 'book' says "let mut scores...", but cargo check
//     //      ...said mut isn't needed, and it does work without it.
//     let scores: HashMap<_, _> =
//         teams.into_iter().zip( initial_scores.into_iter() ).collect();

//     println!("scores, ``{:?}``", scores);
// }



// fn misc01() {
//     let mut scores = HashMap::new();

//     scores.insert( String::from("Blue"), 10 );
//     scores.insert( String::from("Yellow"), 50 );

//     // scores.insert( String::from("foo"), "bar" );  // nope; "mismatched types" error

//     println!("scores, ``{:?}``", scores);
// }
