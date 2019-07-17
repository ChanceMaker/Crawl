//Unit Testing 
pub fn add(a:i32,b:i32)-> i32{
	a+b
}

//this is a really bad adding funciton , its purpose is to fail in this  example
#[allow(dead_code)]
fn bad_add(a:i32,b:i32) -> i32{
	a-b
}

#[cfg(test)]
mod test{
	//Note this usefule idiom: importing names from outer (for mod tests) scpope
}
