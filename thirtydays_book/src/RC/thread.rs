use std::sync::Arc; 
use std::thread; 
use std::time::Duration; 
struct Configuration { 
    setting_a: String, 
    setting_b: u32, 
}

fn main(){
   let config = Configuration {
        setting_a: "example_setting".to_string(),
        setting_b: 42,
   }; 

   let shared_config = Arc::new(config);

   let mut handles = vec![];
   for i in 0..5 {
        let config_clone = Arc::clone(&shared_config);

        let handle = thread::spawn(move || { 
            
            println!("Thread {} started.", i); 
            println!( "Thread {}: Setting A = {}, Setting B = {}", i, config_clone.setting_a, config_clone.setting_b);

            thread::sleep(Duration::from_millis(100)); 
            println!("Thread {} finished.", i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Main tread finished. Config setting B: {}", shared_config.setting_b);
}
