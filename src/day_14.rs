

pub fn part_a(input: &str, count: usize, needle: Option<&str>) -> Vec<i32> {

    // setup anonymous function to convert str to Vec<i32>
    let get_nums = |i: &str| -> Vec<i32> { i.chars().map(|x| x.to_digit(10).unwrap() as i32).collect() };

    // setup the needle to be empty or set to u32s
    let temp = match needle {
        Some(n) => get_nums(n),
        None => Vec::new(),
    };

    // init the two most important pieces, scores and players
    let mut scores = get_nums(input);
    let mut players = [0, 1];

    // do the work to find either a pattern or number of recipes...
    loop {
        // Take care of the case of a specific number of recipes that need to pass
        // _magic number 10 is the count of recipes expected after the previous recipes are found
        if temp.is_empty() && scores.len() == count + 10 {
            return scores[scores.len() - 10..].to_vec();
        }

        // if a needle is present let's break out early if match found
        if !temp.is_empty() {
            if scores.ends_with(&temp) {
                return scores[..scores.len() - temp.len()].to_vec();
            } else if scores[..scores.len()-1].ends_with(&temp) {
                return scores[..(scores.len() - temp.len() - 1)].to_vec();
            }
        }

        // get the player's score from the chain
        let recipe_01 = scores[players[0]];
        let recipe_02 = scores[players[1]];

        // add the two links together
        let append_score = get_nums(&format!("{}", recipe_01 + recipe_02));

        // update chain
        scores.extend(append_score);


        // increase the recipe number by one and cycle to next player index
        players[0] = ((recipe_01 + 1) as usize + players[0]) % scores.len();
        players[1] = ((recipe_02 + 1) as usize + players[1]) % scores.len();
    }
}

#[test]
fn test_modulo() {
    let z = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert_eq!(9 % 9, 0);
    assert_eq!(9 % 10, 9);
    assert_eq!(9 % (9+1), 9);
    assert_eq!(9 % 4, 1);

    // this is reasonable for speed.
    assert_eq!(11 % z.len(), 0);
    assert_eq!(14 % z.len(), 3);

    // this is really really slow :(
    let x = z.iter().cycle().nth(11).unwrap();
    let y = z.iter().cycle().nth(11 + 3).unwrap();
    assert_eq!(*x, 0);
    assert_eq!(*y, 3);
}

#[test]
fn test_part_a() {
    let input = "37";
    let result = part_a(input, 9, None);
    assert_eq!(result, [5, 1, 5, 8, 9, 1, 6, 7, 7, 9]);
}

#[test]
fn test_part_b() {
    let input = "37";
    let result = part_a(input, 25, Some("589"));
    assert_eq!(result.len(), 11);
}

#[test]
fn test_needle_9() {
    let input = "37";
    let result = part_a(input, 10, Some("51589"));
    // println!("{}", result);
    assert_eq!(result.len(), 9);
}

#[test]
fn test_needle_2018() {
    let input = "37";
    let result = part_a(input, 10_000, Some("59414"));
    println!("{:?}", result);
    assert_eq!(result.len(), 2018);
}
