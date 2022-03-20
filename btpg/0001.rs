use std::io;

fn main(){
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    io::stdin().read_line(&mut line).unwrap();
    io::stdin().read_line(&mut line).unwrap();
    let strings: Vec<&str> = line.split('\n').collect();
    let mut sum : i32 = 0;
    sum += strings[0].parse::<i32>().unwrap();
    sum += strings[1].parse::<i32>().unwrap();
    sum += strings[2].parse::<i32>().unwrap();

    if sum >= 80 {
       println!("A"); 
    }
    else if sum >= 75 {
        println!("B+"); 
    }
    else if sum >= 70 {
        println!("B"); 
    }
    else if sum >= 65 {
        println!("C+"); 
    }
    else if sum >= 60 {
        println!("C"); 
    }
    else if sum >= 55 {
        println!("D+"); 
    }
    else if sum >= 50 {
        println!("D"); 
    }
    else {
        println!("F"); 
    }

    
}