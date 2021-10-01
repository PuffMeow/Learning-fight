// let v:Vec<u32> = vec![1,2,3];

#[macro_export]
macro_rules! vec {
  ($($x:expr), *) => {
    {
      let mut temp_vec = Vec::new();

      $(
        temp_vec.push($x);
      )*
      temp_vec
    }
  };
}

// let mut temp_vec = Vec::new();
// temp_vec.push(1);
// temp_vec.push(2);
// temp_vec.push(3);
