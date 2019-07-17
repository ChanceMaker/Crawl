//For loops working on incrementation
fn main(){
	for i in (0..5).step_by(2){
		if i % 2 == 0{
			println!("even {}",i );
		}else{
			println!("odd {}",i );
		}
		
	}
}
