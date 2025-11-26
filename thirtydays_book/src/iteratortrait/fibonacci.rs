struct Fibonacci {
    a: u64,
    b: u64,
}

impl Iterator  for Fibonacci  {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.a == 0 && self.b == 0 {
            self.a = 0;
            self.b = 1;
            Some(0)
        } else {
            let result = self.a;
            let next_val = self.a.checked_add(self.b);
            match next_val {
                Some(val) => {
                    self.a = self.b;
                    self.b = val;
                    Some(result)
                },
                None => {
                    None
                }
            } 
        }
    }
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { a: 0, b: 0 }
    }
}

fn main () {
    let fib = Fibonacci::new();
    let first_ten_fib: Vec<u64> = fib.take(10).collect();
    println!("The first 10 fibonacci numbers are: {:?}", first_ten_fib);
}