

#[test]
fn test_modulo() {
    assert_eq!(9 % 9, 0);
    assert_eq!(9 % 10, 9);
    assert_eq!(9 % (9+1), 9);
    assert_eq!(9 % 4, 1);

    let z = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let x = z.iter().cycle().nth(11).unwrap();
    let y = z.iter().cycle().nth(11 + 3).unwrap();
    assert_eq!(*x, 0);
    assert_eq!(*y, 3);
}

pub fn part_a(input: &str, count: usize) -> String {
    let get_nums = |i: &str| -> Vec<u32> { i.chars().map(|x| x.to_digit(10).unwrap()).collect() };

    let mut scores = get_nums(input); // input.chars().map(|x| x.to_digit(10).unwrap()).collect();

    let mut players = [0, 1];
    loop {
        if scores.len() == count + 10 {
            scores.reverse();
            let result = scores.iter().take(10).map(|i| i.to_string()).collect::<String>();
            return result.chars().rev().collect::<String>();

//
//            let buffer = scores.iter().map(|i| i.to_string()).collect::<String>();
//            match buffer.find(needle) {
//                Some(n) => {
//                    if buffer.len() > n + needle.len() + 9 {
//                        let result: Vec<&str> = buffer.split(needle).collect();
//
//                        return result[1].to_string();
//                    }
//                },
//                None => ()
//            }

            // return format!("Something went horribly wrong.");

        }

        // println!("Scores: {:?}", scores);
//        println!("Player: {:?}", players);
        // get the number of player's index in the chain
        let mut recipe_01 = scores[players[0]];
        let mut recipe_02 = scores[players[1]];

        // add two links together
        let append_score = get_nums(&format!("{}", recipe_01 + recipe_02));
        // println!("Going to append: {:?}", append_score);
        // update chain
        scores.extend(append_score);
        // println!("New Chain: {:?}", scores);

        // increase the number by one
        recipe_01 += 1;
        recipe_02 += 1;
        // println!("RECIPE: {}-{}", recipe_01, recipe_02);

        // update players index
        players[0] = (0..scores.len()).cycle().nth(recipe_01 as usize + players[0]).unwrap() as usize;
        players[1] = (0..scores.len()).cycle().nth(recipe_02 as usize + players[1]).unwrap() as usize;
    }


}

#[test]
fn test_part_a() {
    let input = "37";
    let result = part_a(input, 9);
    assert_eq!(result, String::from("5158916779"));
}