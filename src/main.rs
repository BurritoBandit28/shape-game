use password_validator::{
    check_password_strength,
    PasswordFeedback
};
use crossterm::{
    terminal,
    ExecutableCommand,
    cursor};
use std::{
    fs::File,
    io::{stdout, BufReader, Read, Write},
    fs,
    io
};
use std::f32::consts::PI;
use std::ffi::c_float;
use rand::Rng;
use rand::rngs::mock::StepRng;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use std::{thread, time};
use std::str::FromStr;
use chrono;

pub const PANIC_MESSAGE: &str = "Aaaaaah! panic!!!";
pub static mut USER_NAME : String = String::new();

pub static mut SESSION_SCORE : u32 = 0;


fn main() -> io::Result<()> {
    let mut stdout = stdout();
    fs::create_dir_all("./data/users/").expect(PANIC_MESSAGE);

    println!("Login or sign up");
    let mut lgnsn: String = String::new();
    io::stdin().read_line(&mut lgnsn).expect(PANIC_MESSAGE);

    if lgnsn.trim().to_lowercase().eq("sign up") || lgnsn.trim().to_lowercase().eq("signup") {
        unsafe { sign_up().expect(PANIC_MESSAGE) }
    }
    else if lgnsn.trim().to_lowercase().eq("log in") || lgnsn.trim().to_lowercase().eq("login") {
        unsafe { login().expect(PANIC_MESSAGE) }
    }

    print!("{esc}c", esc = 27 as char);
    fs::create_dir_all(std::format!("./data/users/{}/scores/", unsafe { &USER_NAME })).expect(PANIC_MESSAGE);

    let path: String = std::format!("./data/users/{}/scores/{}.txt", unsafe { &USER_NAME }, chrono::offset::Local::now().to_string().trim().replace(" ", "_").replace(":", "--"));
    let mut file: File = File::create(&path)?;
    unsafe {
        file.write_all(SESSION_SCORE.to_string().as_ref());
        println!("Your final score is : {}\n", SESSION_SCORE)
    }

    println!("View previous scores? (y/n)");
    let mut ps : String = String::new();
    io::stdin()
        .read_line(&mut ps)
        .expect("oopsie tehe");
    if ps.trim().to_lowercase().eq("y") {
        stdout.execute(cursor::MoveUp(1)).expect("tweedle dum");
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown)).expect("tweedle doo");
        let mut scores : Vec<usize> = vec![];
        let paths = fs::read_dir(std::format!("./data/users/{}/scores/", unsafe { &USER_NAME })).unwrap();
        for path  in paths {
            let mut score : String = String::new();
            let file : File = File::open(std::format!("./data/users/{}/scores/{:?}",unsafe { &USER_NAME },&path.as_ref().unwrap().file_name()).replace("\"", ""))?;
            let mut buf_reader = BufReader::new(file);

            buf_reader.read_to_string(&mut score)?;
            scores.append(&mut vec!(usize::from_str(&*score).unwrap()))
        }
        scores.sort();
        scores.reverse();
        for x in scores{
            println!("{}", x)
        }
        Ok(())
    }
    else {
        Ok(())
    }
}

unsafe fn login() -> io::Result<()> {
    println!("\nPlease enter your username");
    let mut user_name: String = String::new();
    io::stdin()
        .read_line(&mut user_name)
        .expect("there was a problem, please try again");
    let path: String = std::format!("./data/users/{}/pswd.txt", user_name.trim().replace(" ", "-"));
    let file: File = File::open(&path)?;

    let mut pswd: String = String::new();
    let mut buf_reader = BufReader::new(file);

    buf_reader.read_to_string(&mut pswd)?;

    let mut input: String = String::new();
    println!("Please enter your password");
    io::stdin()
        .read_line(&mut input)
        .expect("there was a problem");

    if pswd.trim().eq(input.trim()) {
        println!(
            "Hello {}, you have successfully logged in :3",
            user_name.trim()

        );
        USER_NAME = user_name.trim().parse().unwrap();
        play_game();
        Ok(())
    } else {
        println!("Information wrong, please try again");
        login().expect(PANIC_MESSAGE);
        Ok(())
    }
}
unsafe fn sign_up() -> io::Result<()> {
    let mut stdout = stdout();
    println!("\nPlease enter a username");
    let mut user_name: String = String::new();
    io::stdin()
        .read_line(&mut user_name)
        .expect("there was a problem, please try again");
    fs::create_dir_all(std::format!("./data/users/{}", user_name.trim().replace(" ", "-"))).expect(PANIC_MESSAGE);
    let path: String = std::format!("./data/users/{}/pswd.txt", user_name.trim().replace(" ", "-"));
    let mut file: File = File::create(&path)?;

    let mut pswd: String = String::new();

    let mut pswfb: PasswordFeedback = PasswordFeedback {
        message: "Enter a password".to_string(),
        code: 0,
    };
    while !(pswfb.code >= 5) {
        println!(
            "{}\n{}",
            pswfb.message,
            ("#".repeat(pswfb.code as usize)) + ("-".repeat(5 - pswfb.code as usize)).as_str()
        );
        pswd = "".to_string();
        io::stdin()
            .read_line(&mut pswd)
            .expect("there was a problem");
        pswfb = check_password_strength(&pswd.trim().to_string());

        stdout.execute(cursor::MoveUp(3))?;
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
    }

    file.write_all(std::format!("{}", pswd).as_ref())
        .expect(PANIC_MESSAGE);

    println!("Would you like to login? (y/n)");

    let mut res: String = String::new();

    io::stdin().read_line(&mut res).expect("wdfewgrea a");

    if res.trim().to_lowercase().eq("y") {
        login().expect(PANIC_MESSAGE);
        return Ok(());
    }

    Ok(())
}

fn play_game() {
    print!("{esc}c", esc = 27 as char);
    println!(r#"
    {}
               ^
    Circle (A) |
               V
    {}
                  ^
    Rectangle (B) |
                  V
    {}
                 ^
    Triangle (C) |
                 V

    Or Quit"#, circle(), rectangle(), triangle());

    let mut str : String = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("brokey");

    match str.trim().to_lowercase().as_str() {
        "a" => unsafe {play_circle()}
        "b" => unsafe {play_rectangle()}
        "c" => {}
        "quit" => { return;}
        _ => {play_game();return;}
    }
}

fn get_circle_area(radius : u32) -> f32 {
    return 3.14 * (radius as f32 * radius as f32)
}

unsafe fn play_circle() {
    let mut stdout = stdout();
    print!("{esc}c", esc = 27 as char);
    let num: u32 = rand::thread_rng().gen_range(6..20);
    let mut correct_answer : f32 = get_circle_area(num);
    let mut answers : Vec<f32> = vec![correct_answer, correct_answer - 10.0, correct_answer -4.3, correct_answer + 5.2];

    let mut rng = rand::thread_rng();
    let mut irs = Irs::default();

    irs.shuffle(&mut answers, &mut rng);

    println!(r#"{}
    Radius = {}

    Assume Pi as 3.14

    | A: {:.2} | B: {:.2} | C: {:.2} | D: {:.2} |
    Enter your answer."#, circle(), &num, &answers[0],&answers[1],&answers[2],&answers[3]);

    if answers[abcd_to_0123()].eq(&correct_answer) {
        println!("Correct! +2 points!");
        SESSION_SCORE +=2;
        thread::sleep(time::Duration::from_secs(3u64));
        play_game();
        return;
    }
    else {
        println!("wrong please enter again");
        thread::sleep(time::Duration::from_secs(1u64));
        stdout.execute(cursor::MoveUp(2)).expect("tweedle dum");
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown)).expect("tweedle doo");
        if answers[abcd_to_0123()].eq(&correct_answer) {
            println!("Correct! +1 points!");
            SESSION_SCORE +=1;
            thread::sleep(time::Duration::from_secs(3u64));
            play_game();
            return;
        }
        else {
            println!("wrong, you suck");
            thread::sleep(time::Duration::from_secs(3u64));
            play_game();
            return;
        }
    }
}

fn get_rectangle_area(width : u32, length : u32) -> u32 {
    return (width * length)
}

unsafe fn play_rectangle() {
    let mut stdout = stdout();
    print!("{esc}c", esc = 27 as char);
    let length: u32 = rand::thread_rng().gen_range(6..20);
    let width: u32 = rand::thread_rng().gen_range(6..20);
    let mut correct_answer : u32 = get_rectangle_area(length, width);
    let mut answers : Vec<u32> = vec![correct_answer, correct_answer - 4, correct_answer -3, correct_answer + 6];

    let mut rng = rand::thread_rng();
    let mut irs = Irs::default();

    irs.shuffle(&mut answers, &mut rng);

    println!(r#"{}
    length = {}
    width = {}

    | A: {:.2} | B: {:.2} | C: {:.2} | D: {:.2} |
    Enter your answer."#, rectangle(), &length, &width, &answers[0], &answers[1], &answers[2], &answers[3]);

    if answers[abcd_to_0123()].eq(&correct_answer) {
        println!("Correct! +2 points!");
        SESSION_SCORE +=2;
        thread::sleep(time::Duration::from_secs(3u64));
        play_game();
        return;
    }
    else {
        println!("wrong please enter again");
        thread::sleep(time::Duration::from_secs(1u64));
        stdout.execute(cursor::MoveUp(2)).expect("tweedle dum");
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown)).expect("tweedle doo");
        if answers[abcd_to_0123()].eq(&correct_answer) {
            println!("Correct! +1 points!");
            SESSION_SCORE +=1;
            thread::sleep(time::Duration::from_secs(3u64));
            play_game();
            return;
        }
        else {
            println!("wrong, you suck");
            thread::sleep(time::Duration::from_secs(3u64));
            play_game();
            return;
        }
    }
}

fn get_triangle_area(base : u32, height : u32) -> f32 {
    return 0.5 * (base as f32, height as f32)
}

unsafe fn play_triangle() {
    let mut stdout = stdout();
    print!("{esc}c", esc = 27 as char);
    let base: u32 = rand::thread_rng().gen_range(6..20);
    let height: u32 = rand::thread_rng().gen_range(6..20);
    let mut correct_answer : f32 = get_triangle_area(base, height);
    let mut answers : Vec<f32> = vec![correct_answer, correct_answer - 3.8, correct_answer +2.3, correct_answer + 5.3];

    let mut rng = rand::thread_rng();
    let mut irs = Irs::default();

    irs.shuffle(&mut answers, &mut rng);

    println!(r#"{}
    base = {}
    height = {}

    | A: {:.2} | B: {:.2} | C: {:.2} | D: {:.2} |
    Enter your answer."#, triangle(), &base, &height, &answers[0], &answers[1], &answers[2], &answers[3]);

    if answers[abcd_to_0123()].eq(&correct_answer) {
        println!("Correct! +2 points!");
        SESSION_SCORE +=2;
        thread::sleep(time::Duration::from_secs(3u64));
        play_game();
        return;
    }
    else {
        println!("wrong please enter again");
        thread::sleep(time::Duration::from_secs(1u64));
        stdout.execute(cursor::MoveUp(2)).expect("tweedle dum");
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown)).expect("tweedle doo");
        if answers[abcd_to_0123()].eq(&correct_answer) {
            println!("Correct! +1 points!");
            SESSION_SCORE +=1;
            thread::sleep(time::Duration::from_secs(3u64));
            play_game();
            return;
        }
        else {
            println!("wrong, you suck");
            thread::sleep(time::Duration::from_secs(3u64));
            play_game();
            return;
        }
    }
}

fn abcd_to_0123() -> usize {
    let mut str : String = String::new();
    let mut ind : usize = 0;
    io::stdin()
        .read_line(&mut str)
        .expect("brokey");
    loop {
        if str.trim().to_lowercase().as_str().eq("a") {
            ind = 0;
            break
        } else if str.trim().to_lowercase().as_str().eq("b") {
            ind = 1;
            break
        } else if str.trim().to_lowercase().as_str().eq("c") {
            ind = 2;
            break
        } else if str.trim().to_lowercase().as_str().eq("d") {
            ind = 3;
            break
        } else { println!("you didnt enter A, B, C, or D. Please enter again");
            str = "".to_string();
            io::stdin()
                .read_line(&mut str)
                .expect("brokey");
        }
    }
    return ind;
}

fn triangle() -> String {
    let num: i32 = rand::thread_rng().gen_range(0..5);
    return match num {
        0 => {
            r#"
    |\
    | \
    |__\
    "#.to_string()
        },
        1 => {
            r#"
      /|
     / |
    /__|
    "#.to_string()
        },
        2 => {
            r#"
      /\
     /  \
    /____\
    "#.to_string()
        },
        3 => {
            r#"
    \﹉﹉-/
     \  /
      \/
      "#.to_string()
        },
        4 => {
            r#"
    |\
    | \
    | /
    |/
    "#.to_string()
        },
        _ => { String::new() }
    }
}
fn rectangle() -> String {

    let num : i32 = rand::thread_rng().gen_range(0..4);
    return match num {
        0 => {
            r#"
    |﹉﹉﹉|
    |     |
    |_____|
    "#.to_string()
        },
        1 => {
            r#"
    |﹉﹉﹉﹉|
    |________|
    "#.to_string()
        },
        2 => {
            r#"
    |﹉﹉﹉  |
    |       |
    |       |
    |_______|
    "#.to_string()
        },
        3 => {
            r#"
    |﹉﹉|
    |   |
    |   |
    |___|
      "#.to_string()
        },
        _ => { String::new() }
    }
}

fn circle() -> String {

    return r#"
        x  x
     x        x
    x          x
    x          x
     x        x
        x  x         this is a circle believe me
    "#.to_string()

}
