struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with Data `{}`", self.data);
    }
}

fn main() {
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created!.");
    drop(d) //std::mem::drop()
            //at the end of the scope of a variable, drop gets called.
}
