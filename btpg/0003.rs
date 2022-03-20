use std::io;

fn main(){

    let mut arr = vec![vec![0; 105]; 105];
    
    let mut line = String::new();
    io::stdin().read_line(&mut line);

    let v : Vec<&str> = line.split::<&str>(" ").collect();
    let n : usize = v[0].trim().parse::<usize>().unwrap();
    let m : usize = v[1].trim().parse::<usize>().unwrap();

    for i in 0..(n) {
        let mut line = String::new();
        io::stdin().read_line(&mut line);
        let v : Vec<&str> = line.split::<&str>(" ").collect();
        for j in 0..(m) {
            
            arr[i][j] = v[j].trim().parse::<i32>().unwrap();
        }
    }
    for i in 0..(n) {
        let mut line = String::new();
        io::stdin().read_line(&mut line);
        let v : Vec<&str> = line.split::<&str>(" ").collect();
        for j in 0..(m) {
            
            arr[i][j] += v[j].trim().parse::<i32>().unwrap();
        }
    }

    for i in 0..(n) {
        for j in 0..(m) {
            print!("{} ",arr[i][j]);
        }
        println!("");
    }


}