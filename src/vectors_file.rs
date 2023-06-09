pub mod vectors_file {
    fn some_func() {
        // these are from docs
        let v1: Vec<i32> = Vec::new();

        // vec! is macro like doc! in mongodb
        let v2 = vec![1, 2, 3];

        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);

        // reading element
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }
        

        //
        let v = vec![1, 2, 3, 4, 5];

        let does_not_exist = &v[100];
        let does_not_exist = v.get(100);
    
    }
}
