

    pub fn c_to_f_temp() {
        let mut temperature = [1.0f64, 3.0, 10.0, 100.0];

        // it change the array
        temperature
            .iter_mut()
            .for_each(celsius_to_fahrenheit);
        
        println!("{:?}", temperature);

    }   
    fn celsius_to_fahrenheit(celsius: &mut  f64){
        *celsius *= 9.0 / 5.0 + 32.0
    }    







