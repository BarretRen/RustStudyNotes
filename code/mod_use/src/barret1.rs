pub mod barret2 {
    pub struct A {
        pub width: u32,
        pub height: u32,
    }
    impl A {
        pub fn aera(self) -> u32 {
            self.width * self.height
        }
    }
}