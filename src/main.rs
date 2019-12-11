use rand::Rng;

fn main() {
    approximate_karpov();
    statistometer();
}

fn statistometer() {
    let mut attempts: Vec<i64> = vec![];
    let mut sum : i64 = 0;

    let count = 5000;

    for i in 0..count {
        let res = counter_karpov();
        attempts.push(res);
        sum += res;
    }

    let m = (sum as f64) / (count as f64);
    let mut sd = 0.0;

    for i in 0..count {
        sd += ((attempts[i] as f64) - m).powf(2.0);
    }

    println!("So, the E = {} and D = {}", m, (sd / (count - 1) as f64).powf(0.5))
}

fn approximate_karpov() {
    let mut days : f32 = 0.0;

    for i in 1..100 {
        let waiting_for_new_registree = 100.0 / (100 - i) as f32;
        days += waiting_for_new_registree;
        days += 100.0;
    }
    println!("You will be waiting for {} days", days);
}

fn counter_karpov() -> i64 {
    let mut lighted_prisoners = [false; 99];
    let mut registered_prisoners = 0;
    let mut day_counter = 0i64;
    let mut bulb = false;

    let registrator = 99;
    let mut generator = rand::thread_rng();

    while registered_prisoners < 99 {
        let prisoner = generator.gen_range(0, 100);

        if prisoner == registrator && bulb {
            registered_prisoners += 1;
            bulb = false;
        } else if prisoner != registrator && !bulb && !lighted_prisoners[prisoner as usize]{
            lighted_prisoners[prisoner as usize] = true;
            bulb = true;
        }
        day_counter += 1;
    }

    // println!("All is done after {} days", day_counter);
    // println!("Lighted prisoners: {}", lighted_prisoners.len());
    day_counter
}