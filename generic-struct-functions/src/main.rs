// Demonstrate the ability for a generic struct to have several functions with the *same* name but
// with different implementations differentiated by type.

struct MyStruct<T> {
    data: T
}

impl MyStruct<i32> {
    fn x(&self, a: i32) -> &i32 {
        print!("Called x with an i32 with a value of '{}'. ", a);
        println!("Returning '{:#?}'.", self.data);
        &self.data
    }
}

impl MyStruct<char> {
    fn x(&self, a: char) -> &char {
        print!("Called x with a char with a value of '{}'. ", a);
        println!("Returning '{:#?}'.", self.data);
        &self.data
    }
}

impl MyStruct<&str> {
    fn x(&self, a: &str) -> &str {
        print!("Called x with a string reference with a value of '{}'. ", a);
        println!("Returning '{:#?}'.", self.data);
        &self.data
    }
}


fn main() {
    let my_data_i32 = MyStruct { data: 3 };
    my_data_i32.x(7);

    let my_data_char = MyStruct { data: 'a' };
    my_data_char.x('c');

    let my_data_str = MyStruct { data: "This string is specificed when struct is created" };
    my_data_str.x("This string is passed as parameter to the call to the function");

}
