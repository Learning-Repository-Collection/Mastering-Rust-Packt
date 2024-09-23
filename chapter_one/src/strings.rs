

pub fn string_module() {

    // with_capacity() means how many bytes it can hold before reallocating more space
    let mut ynwa = String::with_capacity(23);

    // push_str() appends the string literal to the previously declare ynwa string
    ynwa.push_str("You'll never walk alone");

    // to_owned() is a method that converts the string slice into a String
    // let mut ynwa = "You'll never walk alone".to_owned();

    let home_team = "Liverpool";
    let result = " beat ";
    let away_team = "Manchester United";
    let full_line = home_team.to_owned() + result + away_team;
    println!("{}", full_line);


}

pub fn build_string() {

    let home_team = "Liverpool";
    let result = " beat ";
    let away_team = "Manchester United";

    let home_score = '3';
    let away_score = "-0";

    let mut full_line = format!("{}{}{} ", home_team, result, away_team);

    // add the character to the end of the string
    full_line.push(home_score);
    full_line.push_str(away_score);
    println!("{}", full_line);

    let my_score = 10i32;
    // let mut final_score : u32 = 100;
    let final_score = my_score as u32;
    println!("{}", final_score);

    // float to int
    let pi = 3.14;
    let new_pi = pi as i32; // new_pi = 3
    println!("{}", new_pi)

}
