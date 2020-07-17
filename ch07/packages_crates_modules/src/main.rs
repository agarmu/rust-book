mod toplevel {
    pub mod public_child {
        pub fn random () {
            println!(".>(ASDFGHJKL")
        }
        
    }
}

fn main() {
   toplevel::public_child::random()
}
