mod option_description {
    use crate::argument::OptionDescription;

    #[test]
    fn it_works() {
        let desc = OptionDescription::new("option")
            .long("hello");
    }

    #[test]
    fn it_works_complex() {
        let desc = OptionDescription::new("option")
            .long("hello".to_owned());
    }
}
