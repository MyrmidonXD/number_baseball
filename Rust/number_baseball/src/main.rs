extern crate rand;

use std::io;
use std::io::*;
use rand::Rng;

fn main() {

    let mut playing = true;

    while playing {

        let target_num: Vec<u32> = gen_rand_num();
        playing = num_baseball(&target_num);

    }


}

fn gen_rand_num() -> Vec<u32> {
    let mut num = Vec::<u32>::new();
    let mut i = 0;
    
    while i < 4 {
        let mut duplicated = false;
        let x: u32 = rand::thread_rng().gen_range(0, 10);

        for digit in num.iter() {
            if digit == &x {
                duplicated = true;
                break;
            }
        }

        if !duplicated {
            num.push(x);
            i = i + 1;
        }
    }

    num
}


fn num_baseball(num: &Vec<u32>) -> bool {

    println!("");
    println!("============================================================");
    println!("각 자리가 서로 다른 숫자로 이루어진 네 자리 수를 입력하세요!");
    println!("============================================================");

    loop {
        let mut guess = String::new();
        let mut guess_int = Vec::<u32>::new();
        let mut invalid_input = false;
        println!("\n추측한 숫자를 입력해주세요.");
        
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("입력을 받는 데에 실패했습니다.");

        guess = guess.trim()
                .to_string();

        if guess.len() != 4 {
            invalid_input = true;
            //println!("Error 1: 길이 다름");
        }
        
        for c in guess.chars() {
            if c.is_digit(10) {
                let x: u32 = c.to_digit(10)
                            .unwrap();

                if !guess_int.contains(&x) {
                    guess_int.push(x);
                } else {
                    invalid_input = true;
                    //println!("Error 2: 숫자 중복");
                    break;
                }

            } else {
                invalid_input = true;
                //println!("Error 3: 숫자 이외의 문자");
                break;
            }
        }

        //println!("당신의 입력 : {}", guess);

        if invalid_input {
            println!("입력이 잘못되었습니다. 입력은 각 자리가 서로 다른 숫자로 이루어진 4자리의 정수여야 합니다.");
            continue;
        }

        let mut strike = 0;
        let mut ball = 0;
        let mut index = 0;

        while index < 4 {
            let digit = guess_int[index]; 
            if num.contains(&digit) {
                if num[index] == digit {
                    strike = strike + 1;
                } else {
                    ball = ball + 1;
                }
            }
            index = index + 1;
        }

        if strike == 4 {
            println!("");
            println!("정답입니다! 답은 {}{}{}{}입니다.", num[0], num[1], num[2], num[3]);
            println!("");

            println!("다시 플레이하시겠습니까? (Y/N)");

            let mut string = String::new();
            io::stdin().read_line(&mut string)
                .ok()
                .expect("입력을 받는 데에 실패했습니다.");
            let c = string.chars()
                    .next()
                    .unwrap();
            if c == 'Y' || c == 'y' {
                break;
            } else {
                return false;
            }
        } else {
            println!("{} 스트라이크, {} 볼입니다.", strike, ball);
        }
    }

    true
}
