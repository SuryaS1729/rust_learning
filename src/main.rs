
//****EVEN OR ODD */

// fn main(){
//     let ans= is_even(201);
//     println!("{}", ans)

// }

// fn is_even(num: i32)-> bool{
//     if num%2==0{
//         return true
//     }
//     return false
// }

//FIBONACCCI


// // 0,1,1,2,3,5,8, fib of 0 is 0 fib of 1 is 1 fib of 2 is 1 fib of 3 is 2
//f(0)=0,f(1)=1,f(2)=1,f(3)=2

//  fn main() {
//      println!("{}", fib(5))
//  }

//  fn fib(num: u32)->u32{
//         let mut first = 0;
//         let mut second=1;

//         if num==0{
//             return first
//         }
//         if num==1{return second}

//         for _ in 0..num-1{
//             let temp= second;
//             second = second+first;
//             first = temp;

//         }return second
//  }

//0,1,1,2,3,5,8,13,21,34
//0,1,2,3,4,5,6,07,08,09


//******* THIS IS HOW TO COUNT THE NO OF CHARACTERS IN A STRING */
// fn main(){
//     let name= String::from("sursdssssssssya");
//     let length=get_str_len(name);
//     println!("the length of the string is {}",length)
// }

// fn get_str_len(str: String)->usize{
//     str.chars().count()
// }


//STRUCTS

// use core::num;

// struct User{
//         active: bool,
//         username: String,
//         email: String,
//         sign_in_count: u64,
// }

// fn main(){
//     let user1= User{
//         active: true,
//         username: String::from("sahithi"),
//         email: String::from("sahithinatva28@gmail.com"),
//         sign_in_count:1,


//     };

//     println!("User1 username is {}", user1.username)
// }

//implementing functions on structs

// struct Rect{
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self)->u32{
//         self.width/self.height
//     }
// }

// fn main(){
//     let rect1=Rect{
//         width:100,
//         height:20,
//     };
//     print!("the area of the rectangle rect1 is {}",rect1.area())
// }

// next up enums

//enums

// enum SahithiBestFriends{
//     Harshini,
//     Nithin,
//     Mahathi,
//     Abhilash,
// }

// enum Shape{
//     Rectangle(f64,f64),
//     Circle(f64),
// }

// fn main(){
//     let rect=Shape::Rectangle(1.0, 2.0);
//     calc_area(rect);
    
//     let circ= Shape::Circle(1.0);
//     calc_area(circ);
// }

// fn calc_area(shape: Shape)->f64{
//     let area = match shape{
//         Shape::Rectangle(a, b)=>a*b,
//         Shape::Circle(r)=> 3.14*r*r

//     };
//     return area;
// }

//OPTION ENUM

// fn main(){
//     let index=find_first_a(String::from("preesat"));

//     match index {
//         Some(value)=> print!("index is {}", value),
//         None=> print!("a not found"),
//     }
// }

// fn find_first_a(s: String)->Option<i32>{
//     for(index,char) in s.chars().enumerate(){
//         if char=='a'{
//             return Some(index as i32)// this bit might be an iterator which will come up later, but the point is to understand the option enum of it returning both Some and None
//         }
//     }
//     return  None;
// }

//RESULT ENUM



//Importing packages or crates in rust
// cargo add chrono
// use chrono::{Local};
// fn main(){
//     let now= Local::now();
//     print!("{}",now)
// }

// use std::fs::read_to_string;

// fn main(){
//     let result = read_to_string("a.txt");

//     match result{
//         Ok(data)=>print!("{}", data),
//         Err(err)=>print!("error while reading the file")
//     }
// }

//compsare ownership and garbage coolector
//SCOPE IN RUST
// fn main() {
//     let x = 10; // x is valid from here
//     {
//         let y = 20; // y is valid only within this inner block
//         println!("Inside inner block: x = {}, y = {}", x, y);
//     } // y goes out of scope here
//     // println!("{}", y); // This would cause a compile error
//     println!("Outside inner block: x = {}", x); // x is still valid
// }

//RUST PART 2

//COLLECTIONS

//VECTORS

// use std::vec;

// fn main() {
//     let mut vec:Vec<i32> = vec![1,2,3,4,10,32];
//     vec.push(1);
    
//     let ans = even_filter(&vec);
//     println!("{:?}", vec);
//     println!("{:?}", ans)

// }

// fn even_filter(vec: &Vec<i32>)->Vec<i32>{ 

//     let mut new_vec= Vec::new();

//     for val in vec{
//         if val%2==0{
//             new_vec.push(*val);
//         }
//     }
//     return  new_vec;
// }


//Hashmaps

// use std::collections::HashMap;

// fn main(){
//     let mut users= HashMap::new();

//     users.insert("sahithi", "GOOD");
//     users.insert("kamala", " a stupid WITCH WOMAN");

//     let first_user_type = users.get("kamala");
//     match first_user_type{
//         Some(value)=>print!("USER IS {}",value),
//         None=>print!("user not found")
//     }
// }

// use std::collections::HashMap;

// fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> 
// {
//     let mut hm = HashMap:: new();
//     for (key, value) in vec {
//     hm.insert(key,value);}
//     return hm;
// }
    
// fn main(){
//     let input_vec= vec![(String::from("sahithi"),23),(String::from("surya"),23)];
//     let hm = group_values_by_keys(input_vec);

//     println!("{:?}", hm)
// }

//ITERATORS

//#1 iterating using for loops
// fn main(){
//     let nums = vec![1,2,3,4 ];

//     for everybumber in nums{
//         print!("{}",everybumber)
//     }
// }
//#2 iterating after creating an iterator

// fn main(){
//     let nums = vec![1,2,3,4 ];
//     let iter = nums.iter();
//     for everybumber in iter{
//         print!("{}",everybumber)
//     }
// }

//dereferencing in rust and borrowing
// fn main(){
// let mut x = 10;
// let y = &mut x;  // y is a mutable reference to x

//     *y=23;   // *y dereferences y to access and modify the value of x
// println!("{}", x);  // Prints 20
// }

//#3 ITERMUT- you can mutate the values in the collection while iterating, okay this iterator borrows the values
//of the variable

// fn main(){
//     let mut v1 =vec![1,2,3];

//     let v1_iter = v1.iter_mut();

//     for val in v1_iter{
//         *val = *val +2
//     }

//     print!("{:?}", v1)
// }



//#4 IntoIter
// //this iterator takes ownership of the variable which means you can no longer use it lateron
// fn main(){

//     let mut v1 = vec![1,2,3];

//     let iter= v1.into_iter();

//     for val in iter{
//         print!("{}", val)
        
//     }
//     // print!("{:?}",v1) you cant do this as v1s ownership has been taken and it has been thrown out by the time the code reaches here.
// }
// the normal for loop uses into iter under the hood

// fn main(){
//     let mut v1=vec![1,2,3,4];

//     for val in v1{
//         print!("{}",val)
//     }

//     print!("{:?}",v1)//see there is an error here, you cannot use this value as the into inter under the hood has taken the ownership instead of just borrowing 
// }
//methods on iterators
//CONSUMING adaptors example sum()

// fn main(){

//     let v1= vec![1,2,3,4];
//     let v1_iter= v1.iter();

//     let sum: i32= v1_iter.sum();
// //this is a consuming adaptor as the .sum consumes the v1_iter and you cannot use the v1_iter later on
//     // print!("{:?}",v1_iter) this line wont work as the value is borrowed into the sum variable now
//     print!("{}", sum)
// }


// iterator adaptor, doesnt consume the iterator but creates a fresh iterator and then uses it
//examples are map and filter
// fn main(){
//     let v1 = vec![1,2,3];
//     let iter= v1.iter();
//     let iter2 =iter.map(|x| x+1);

//     for i in iter2{
//         print!("{}",i)
//     }

// // }
// fn main(){
//         let v1 = vec![1,2,3,40];
//         let iter= v1.iter();
//         let iter2 =iter.filter(|x| *x%2==0);
    
//         for i in iter2{
//             println!("{}",i)
//         }
    
//     }
// //write the logic to first filter all odd values then double each value and create a new vector
// fn main(){
//     let v1= vec![1,2,3,4,5,6,7,8];
//     let iter=v1.iter();
//     let iter2 = iter.filter(|x| *x%2 ==1);
//     let iter3 = iter2.map(|x| x*2);

//     let mut new_vec = Vec::new();

//     for i in iter3{
//         new_vec.push(i);
//     }
//     println!("{:?}", new_vec)
// }
//OR
// fn main(){
//     let v1= vec![1,2,3,4,5,6,7,8,9,10,11];
//     let iter=v1.iter().filter(|x| *x%2 ==1).map(|x| x*20);
    
//     let v2: Vec<i32> = iter.collect();


//     println!("{:?}", v2)
// }

//Strings vs Slices

//Creating a String
// fn main(){
//     let name = String::from("sahithi");
//     println!("{}",name)
// }
//mutating a string
// fn main(){
//     let mut name = String::from("sahithi");
//     name.push_str(" loves surya");
//     println!("{}",name)
// }
//deleting from a string

// fn main(){
//     let mut name = String::from("sahithi");
//     name.push_str(" likes surya");
//     println!("{}",name);
//     name.replace_range(7..name.len(), "");
//     println!("{}",name);
//     name.push_str(" likes abhilash");
//     println!("{}",name)
// }


//see how the scope is running in rust
// use core::num;

// fn main(){
//     let x = 10;
//     let y= addten(x);
//     print!("{:?}",y)
// }

// fn addten(y: i32)->i32{
//     y+10
// }

// //trying to understand string slices.
// fn main(){
//     let greeting = String::from("hello sahithi");
//     let her_name= &greeting[6..13];

//     println!("{}",her_name)
// }    

//TRAITS- very good example

// use std::fmt::format;

// trait Summary{
//     fn summarise(&self)-> String{
//         return String::from("default string");
//     }
// }

// struct User{

//     name: String,
//     age: u32

// }

// struct Fix;
// impl Summary for Fix{}
// struct Education{
//     college_name: String
// }
// impl Summary for User{
//     fn summarise(&self)-> String {
//         return format!("Her name is {} and she is {} years old", self.name,self.age);
//     }
// }

// impl Summary for Education{
//     fn summarise(&self)-> String {
//         return format!("the college she went to is {}", self.college_name)
//     }
// }
// fn main(){
//     let user = User{
//         name: String::from("sahithi"),
//         age: 23
//     };
//     let college_name= Education{
//         college_name:String::from("NIFT")
//     };
//     println!("{}",user.summarise());
//     println!("{}",college_name.summarise());
//     let f=Fix;
//     notify(f)
// }

// fn notify(u: impl Summary){
//     println!("{}",u.summarise())
// }
//you can implement the trait for any struct and call the function defined in the impl on any struct



//LIFETIMES

// fn main(){
//     let ans = longest(String::from("sahithi"), String::from("saaket surya"));
//     print!("{}",ans)
// }

// fn longest(a:String, b:String)->String{
//     if a.len()>b.len(){
//         return a
//     }else {
//         return b
//     }
    
// }
struct User{
    name: &str
}
fn main(){

    let name = String::from("sahithi");
    let user= User{
        name: &name
    }
}