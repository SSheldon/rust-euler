/*
Our original system of equations is:
	a^2+b^2=c^2
	a+b+c=1000

So we know:
	c=1000-a-b
And:
	a^2+b^2=(1000-a-b)^2
	       =1000^2-2000(a+b)+(a+b)^2
	              -2000a-2000b+a^2+2ab+b^2
	0=1000^2-2000a-2000b+2ab
	b(2000-2a)=1000^2-2000a
	b=(1000^2-2000a)/(2000-2a)
	 =(1000(1000-2a))/(2(1000-a))

Since a, b, and c are natural numbers, we must find a natural number a
such that b is also a natural number
*/
fn main() {
	let s = 1000;
	let a = range(1, s).find(|&x| (s*(s-2*x)) % (2*(s-x)) == 0).unwrap();
	let b = (s*(s-2*a)) / (2*(s-a));
	let c = s - a - b;
	println!("{}", a * b * c);
}
