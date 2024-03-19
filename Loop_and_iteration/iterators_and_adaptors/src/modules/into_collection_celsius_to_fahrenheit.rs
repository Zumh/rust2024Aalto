

    fn celsius_to_fahrenheit(f: f64) -> f64 {
        f * 9.0 / 5.0 + 32.0
    }

    pub fn c_to_f_map() {
        let celsiuses = [1.0f64, 3.0, 10.0, 100.0];
        let fahrenheits = celsiuses
            .into_iter()
            .map(celsius_to_fahrenheit)
            .collect::<Vec<f64>>();

        println!("degrees Celsius: {celsiuses:?}");
        println!("degrees Fahrenheit: {fahrenheits:?}");
    }   


