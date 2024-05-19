struct CustomSmartPointers {
    data: String,
}

impl Drop for CustomSmartPointers {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointers with data: {}", self.data);
    }
}

fn main() {
    let ap = CustomSmartPointers {
        data: String::from("AP"),
    };

    let pps = CustomSmartPointers {
        data: String::from("PPS"),
    };
    // ap.drop(); // Not allowed
    drop(ap); // Different from `Drop` method in CustomSmartPointers Struct.
    println!("Created CustomSmartPointers");
}
