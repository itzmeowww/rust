use std::io;

fn main(){
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("");
    let n = inp.trim().parse::<i32>().unwrap();
    
    inp = String::new();

    for _i in 1..(n+1) {
        io::stdin().read_line(&mut inp).expect("");
    }

    let mut v : Vec<&str> = inp.split('\n').collect();
    v.pop();
    
    let mut minn : i32 = v[0].trim().parse().unwrap();
    let mut maxx : i32 = v[0].trim().parse().unwrap();

    for x in v.into_iter() {
        minn = minn.min(x.parse().unwrap());
        maxx = maxx.max(x.parse().unwrap());
    }
   

    println!("{}",minn);
    println!("{}",maxx);

}