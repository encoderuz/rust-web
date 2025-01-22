pub trait Create{
    fn create(&self, title: &str){
        println!("Creating a new to-do: {}", title);
    }
}