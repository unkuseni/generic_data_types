fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest(&char_list));
    
}
// This is a test for finding the largest number in a list
fn _largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// This is a test for finding the largest char in a list
fn _largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// We can also see that the largest fn is also a duplicate code we can reduce this to one fn, this limit the code to types that implement the Copy trait  and PartialOrd traits
fn largest<T>(list: &[T]) -> T 
where T: PartialOrd + Copy {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// TRAITS 
// Imagine you have different kinds of toys that can talk. Some toys are big and can tell long stories. Some toys are small and can only say a few words. You want to make a game where you can hear what all the toys have to say. To do this, you need a way to ask each toy to tell you something. You can use a magic word to do this, and the magic word is the same for all the toys. The magic word is "summarize". When you say "summarize" to a toy, it will tell you something about itself. For example, if you say "summarize" to a big toy that is a news reporter, it will tell you what news story it has. If you say "summarize" to a small toy that is a bird, it will tell you if it tweeted something new or repeated something old. This way, you can hear from all the toys and have fun with them.