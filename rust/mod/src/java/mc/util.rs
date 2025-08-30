pub mod math;

rosttasse::bind! {
    use net.minecraft.util;

    enum ActionResult {
        PASS = "Pass",
        SUCCESS = "Success",
    }

    impl Hand {}

    impl Identifier {
        fn of(mod_id: String, id: String) -> Self;
    }
}
