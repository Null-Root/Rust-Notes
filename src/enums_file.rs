pub mod enums_file {
    // enums, pretty much the same as other languages
    enum ThisIsEnum {
        A1,
        B2,
        C3,
    }

    struct SomeStructAgain {
        enum_prop: ThisIsEnum,
        another_prop: String,
    }

    enum IpAddr {
        V4(String),
        V6(String),
    }

    

    fn some_func() {
        let s1 = SomeStructAgain {
            enum_prop: ThisIsEnum::A1,
            another_prop: String::from("sfsfwefwfwe"),
        };

        let s2 = SomeStructAgain {
            enum_prop: ThisIsEnum::C3,
            another_prop: String::from("etgbrtgwfc"),
        };


        // from docu
        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));
    }
}

