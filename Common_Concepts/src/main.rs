fn main() {
    //println!("Hello, world!");
    //var_and_mut();
    //shadowing();
    //data_types();
    tuple_fn();

}
/*Variables are default immutable when used 
and the keywork mut should be used to override
this default behavior.*/
fn var_and_mut(){
	let mut x = 5;
	println!("The value of x is : {}",x);
	x = 6;
	println!("The value of x is : {}", x);
}
/*Shadowing is done when the first variable 
is "shadowed" by the second, this means the
second variable's value is what appears when 
the variable is used. This is how rust avoids 
using a garbage collector. Basically the value 
is what was last referenced when "let x" is
used.*/
fn shadowing(){
	let x = 5; 
	let x = x + 1;
	let x = x * 2;
	const MAX_POINTS: u32 = 100_000;

	println!("\nThe value of x is : {} \nconstant value {}	", x,MAX_POINTS);

	//shadowing additional example which allows
	//us to not have to change names due to types 
	//i.e. space_str space_num 
	let spaces = "    ";

	let spaces = spaces.len();

	println!("Spaces printout : {}", spaces);

}
fn data_types(){
	let guess: u32 = "42".parse().expect("Not a number!");	
	let tup: (i32,f64,u8) = (500,6.4,1);

	println!("GUESS {}", guess);
	println!("Tuple :");
	println!("{}", tup.0);
	println!("{}", tup.1);
	println!("{}", tup.2);
}

//Tuples can be used as function arguments and as return values
fn reverse(pair: (i32,bool)) -> (bool,i32){
	//'let' can be used to bind the memeber of a tuple to variables
	let (integer,boolean) = pair;
	(boolean,integer)
}
#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);


fn tuple_fn(){
	//A tuple with a bunch of different types
	let long_tuple = (1u8,2u16,3u32,4u64,
					  -1i8,-2i16,-3i32,-4i64,
					  0.4f32,0.2f64, 'a',true);
	//Values can be extracted from the tuple using tuple indexing
	println!("long tuple first value: {}", long_tuple.0);
	println!("loing tuple second value: {}", long_tuple.1);

	//Tuples can be tuple members
	let tuple_of_tuple = ((1u8,2u16,2u32),(4u64,-1i8),-2i16);

	//Tupels are printable
	println!("tuple of tuples: {:?}", tuple_of_tuple );

	//But Long tuples cannot be printed
	//let too_long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12,13);
	//println!("too Long tuple: {:?}",too_long_tuple);
	//TODO ^ Uncomment the above 2 Lines to see the compiler error
	//there is not default print method for this too-long-tuple method one either has to 
	//be made for it 

	let pair = (1,true);
	println!("Pair is {:?}",pair );
	println!("the reversed pair is {:?}",reverse(pair));

	//To create one element tuples, the comma is required to tell them apart
	//from a literal surronded by parentheses
	println!("one element tuple: {:?}", (5u32,));
	println!("just an integer: {:?}", (5u32));

	//tuples can be destructed to create bindings
	let tuple = (1,"hello",4.5,true);

	let (a,b,c,d) = tuple;

	println!("{:?},{:?},{:?},{:?}", a,b,c,d);

	let matrix = Matrix(1.1,1.2,2.1,2.2);
	println!("{:?}", matrix);
}



