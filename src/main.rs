fn main() {
    struct Gun{
        _name: String,
        _type: String,
        _energy: u64,
        _num_of_rounds_per_mag: i32,
        _total_rounds:i32
    }
    impl Gun {
        fn reload(&mut self) {
            if self._total_rounds ==0 {
                println!("not enough ammo");
            }
            else if self._num_of_rounds_per_mag == 30 {
                return ();
            }
            else {
                self._num_of_rounds_per_mag = 30;
                self._total_rounds -= 30;
            }
        }

        fn fire(&mut self){
            if self._num_of_rounds_per_mag == 0 {
                self.reload();
            }
            else {
                self._num_of_rounds_per_mag -=1;
            }
        }
    }

    let mut gun1 = Gun{
        _name: String::from("AK-47"),
        _type: String::from("Primary"),
        _energy: 2000,
        _num_of_rounds_per_mag: 0,
        _total_rounds:30
    };
    // gun1.reload();
    gun1.fire();
    println!("{}",gun1._num_of_rounds_per_mag);
}
