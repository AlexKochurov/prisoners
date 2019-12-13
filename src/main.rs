use rand::Rng;

fn main() {
    statistometer();
}

fn statistometer() {
    let mut attempts: Vec<i64> = vec![];
    let mut sum : i64 = 0;

    let count = 10000;

    for _ in 0..count {
        let res = dynamic_counter();
        attempts.push(res);
        sum += res;
    }

    let m = (sum as f64) / (count as f64);
    let mut sd = 0.0;

    for i in 0..count {
        sd += ((attempts[i] as f64) - m).powf(2.0);
    }

    println!("So, the E = {} and D = {}", m, (sd / (count - 1) as f64).powf(0.5));
    // println!("So, the E = {}", m);
}

#[allow(dead_code)]
fn approximate_karpov() {
    let mut days : f32 = 0.0;

    for i in 1..100 {
        let waiting_for_new_registree = 100.0 / (100 - i) as f32;
        days += waiting_for_new_registree;
        days += 100.0;
    }
    println!("You will be waiting for {} days", days);
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn counter_my_7() -> i64 {
    let mut logs: [[i32; 7]; 7] = [
        [1, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 1],
    ];

    let mut days: i64 = 0;
    let mut bulb = false;
    let mut current_prisoner: i64 = 0;
    let mut generator = rand::thread_rng();

    while days < 100 || !check_logs(&logs) {
        current_prisoner = generator.gen_range(0,7);

        if bulb {
            logs[current_prisoner as usize][(days % 7) as usize] += 1;
        }

        bulb = days % 7 == current_prisoner;
        days += 1;
    }

    // println!("logs: {:?}", logs);

    days
}

#[allow(dead_code)]
fn check_logs(logs: &[[i32; 7]; 7]) -> bool {
    let mut done: bool;

    for i in 0..7 {
        done = true;

        for j in 0..7 {
            if logs[i][j] == 0 {
                done = false;
                break;
            }
        }

        if done {
            return true;
        }
    }

    false
}

#[allow(dead_code)]
fn counter_my_100() -> i64 {
    let mut logs: [[i64; 100]; 100] = [[0; 100]; 100];

    for i in 0..100 {
        logs[i][i] = 1;
    }

    let mut days: i64 = 0;
    let mut bulb = false;
    let mut current_prisoner: i64 = 0;
    let mut generator = rand::thread_rng();

    while days < 1000 || !check_logs_100(&logs) {
        current_prisoner = generator.gen_range(0,100);

        if bulb {
            logs[current_prisoner as usize][(days % 100) as usize] += 1;
        }

        bulb = days % 100 == current_prisoner;
        days += 1;
    }

    // println!("logs: {:?}", logs);

    days
}

#[allow(dead_code)]
fn check_logs_100(logs: &[[i64; 100]; 100]) -> bool {
    let mut done: bool;

    for i in 0..100 {
        done = true;

        for j in 0..100 {
            if logs[i][j] == 0 {
                done = false;
                break;
            }
        }

        if done {
            return true;
        }
    }

    false
}

#[allow(dead_code)]
fn dynamic_counter() -> i64 {
    let mut days: i64 = 0;
    let mut contractor_log = [100; 100];
    let mut generator = rand::thread_rng();

    let mut registered_prisoners = 0;
    let mut current_prisoner: i64;
    let mut registrator = 1000;
    let mut bulb = false;

    // stage one
    for i in 0..100 {
        current_prisoner = generator.gen_range(0, 100);
        if registrator == 1000 {
            if check_prisoners(current_prisoner, &contractor_log) {
                registrator = current_prisoner;
            } else {
                contractor_log[i] = current_prisoner;
                registered_prisoners += 1;
            }
        }
        days += 1;
    }

    // registrator = 0;
    // contractor_log[0] = 0;
    // registered_prisoners += 1;

    // println!("Registrator: {}", registrator);
    // println!("Prisoners:");
    // for i in 0..100 {
    //     if contractor_log[i] == 100 {
    //         break;
    //     }
    //     println!("{}", contractor_log[i]);
    // }

    // stage two
    while registered_prisoners < 100 && days < 100000 {
        current_prisoner = generator.gen_range(0, 100);

        if current_prisoner == registrator && bulb {
            registered_prisoners += 1;
            bulb = false;
            // println!("Registered {}th!     Day {}\n", registered_prisoners, days);

        } else if current_prisoner != registrator && !bulb && !check_prisoners(current_prisoner, &contractor_log) {
            // println!("Found -> {}.         Day {}", current_prisoner, days);
            contractor_log[registered_prisoners] = current_prisoner;
            bulb = true;
        }
        days += 1;
    }

    days -= 1;
    // println!("Done! Days: {}", days);
    days
}

fn check_prisoners(current_prisoner: i64, registered_prisoners: &[i64; 100]) -> bool {
    for i in 0..100 {
        if current_prisoner == registered_prisoners[i] {
            return true;
        }
    }
    false
}