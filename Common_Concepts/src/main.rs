fn main() {
    //println!("Hello, world!");
    var_and_mut();
    shadowing();
    data_types();
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




