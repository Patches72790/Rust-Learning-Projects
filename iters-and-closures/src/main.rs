use std::collections::HashMap;
use std::thread;
use std::time::Duration;

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        let value: HashMap<u32, u32> = HashMap::new();
        Cacher { calculation, value }
    }

    fn value(&mut self, arg: u32) -> u32 {
        let value = self.value.get(&arg);
        match value {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let mut c = Cacher::new(|x: u32| -> u32 { x + 1 });
    println!("Value from cacher: {:?}", &c.value(3));
    println!("Value from cahcer: {:?}", c.value(4));

    generate_workout(24, 20);
}
