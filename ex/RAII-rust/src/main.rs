
    use std::time::Instant;
    use std::time::Duration;
    use std::thread::sleep;
    
    pub struct Timer {
        label: String,
        start: Instant,
    }

    impl Timer{
        pub fn new(label: String) -> Self {
            Timer { label, start: Instant::now() }
        }
    }

    impl Drop for Timer {
        fn drop(&mut self) { 
            println!("Elapsed time for {}: {:?}", self.label, Instant::now()-self.start);
        }
    }


fn main() {
    let _t = Timer::new("l1".to_string());
    for i in 1..5 {
        let _t2 = Timer::new("l2".to_string());
        sleep(Duration::new(1,0));
        println!("Ciclo #{}", i);
        if i == 3 { panic!("Errore grave"); }
    }
}
