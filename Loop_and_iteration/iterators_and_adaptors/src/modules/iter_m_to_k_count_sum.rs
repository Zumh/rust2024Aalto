fn miles_to_kilometers(miles: &f64) -> f64 {
    miles * 1.609344
  }


  /*
  Some other consuming methods than collect are for example the for_each that works like for loop, 
  nth to get the nth value, and count that computes the amount of items in the iterator 
   Rust also provides specialized consuming methods for numerical iterators, such as sum and product.
   */
  pub fn iter_m_to_k_count_sum() {
      let miles = [1.0f64, 3.0, 10.0, 100.0];
      let number_of_distances = miles
          .iter()
          .count();
      let total_kilometers = miles
          .iter()
          .map(miles_to_kilometers)
          .sum::<f64>(); // we need to provide the return type explicitly like with collect
      println!("{total_kilometers} kilometers in {number_of_distances} trips");
  }
  