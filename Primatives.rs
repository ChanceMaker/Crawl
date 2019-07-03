//Primatives
/*
Scalar types
	Signed integers i8,i16,i32,i64,i128 and isize(pointer size)
	unsigned integers 8 ->128 byte
	floating point:f32,f64
	char 'a' 4 bytes each
	bool either true or false 
	unit type () ,(only type is an empty tuple)
Compound Types
	arrays like [1,2,3]
	tuples like (1,true)

*/

fn main(){
	//Variables can be type annotated
	let logical: bool = true;

	let a_float:f64 = 1.0;
	let default_integer = 7;

	//A type can also be inferred from context
	let mut inferred_type = 12;//Type i64 is inferred from another line
	inferred_type = 4294967296 as i64;//Type i64 is inferred from another line

	//a mutable variable's value can be changed
	let mut mutable = 12;
	mutable = 21;
	println!("Mutable Value after the change to 21: {}",mutable );
	//Error! The type of a variable can't be changed
	//mutable = true (this cannot be doen)
	//variables can be overwitten with shadowing.
	let mutable = true;//act of shadowing requires the use of "let"
	println!("Mutable Value after the change to boolean: {}",mutable );



}


// fn typeid<T: std::any::Any>(_: &T) {
//     println!("{:?}", std::any::TypeId::of::<T>());
// }