fn main() {

    let mut sum = 0;
    for n in 1..1000 {
    //println!("{}", n);
      if n % 3 == 0 || n % 5 == 0 {
        sum += n;
        println!("{}", n);
		}
	}
	println!("{}", sum);
}