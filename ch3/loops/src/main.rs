fn main() {
    let mut counter1: i32 = 0;
    let mut counter2: i32 = 0;

    'outerloop: loop {
        println!("outer loop: {counter1}");
        counter1 += 1;

        loop {
            println!("inner loop: {counter2}");
            counter2 += 1;

            if counter2 == 10 {
                break 'outerloop;
            }
        }
    }
}
