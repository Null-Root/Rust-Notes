pub mod structs_file {
    // making this struct "cloneable"
    #[derive(Clone)]
    struct SampleStruct {
        user: String,
        a: i32,
        b: f32,
    }

    // implementing SampleStruct Methods (Associated Functions)
    impl SampleStruct {
        // mut keyword used only for mutation operations
        // format is similar to python, method_name(&self, params...)
        fn set_user(&mut self, value: String) {
            self.user = value
        }
    }

    // can have as many impl blocks
    impl SampleStruct {
        fn get_user_1(&self) -> String {
            return self.user.clone();
        }
        fn get_user_2(&self) -> &str {
            return &self.user;
        }
    }

    // setting up static functions
    impl SampleStruct {
        fn static_shit(param1: u32, param2: u32) -> u32 {
            return param1 + param2;
        }
    }

    // these are like interfaces
    trait SomeTrait {
        fn trait_1(&self);
        fn trait_2(&self, param1: i32);
    }

    impl SomeTrait for SampleStruct {
        fn trait_1(&self) {
            println!("This is trait 1 implemented");
        }

        fn trait_2(&self, param1: i32) {
            println!("This is param1 {}", param1);
        }
    }
}
