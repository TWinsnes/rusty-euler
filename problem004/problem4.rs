use std;
//////////////////////////////////////////////////////////
//
// Problem 4:
// Find the largest palindrome made from the product of 
// two 3-digit numbers.
//
//////////////////////////////////////////////////////////


fn main()
{
	let result : int = solution(3);
	std::io::println(#fmt("Solution: %d", result));
}

fn IsPalindrome(x : int) -> bool
{
	let vector : [char] = str::to_chars(int::str(x));
	let revVect : [char] = vec::reversed::<char>(vector);

	if( vector == revVect)
	{
		//std::io::println(#fmt("%d is a palindrome", x));
		ret true;
	}
	//std::io::println(#fmt("%d is not a palindrome", x));
	ret false;
}

fn solution(n: int) -> int
{	

	let largest : int = 1;
	let smallest : int = 1;
	let pal : int = 0;

	// get largest n-digit number
	let x = 0;
	while (x < n)
	{
		largest *= 10;
		x+=1;
	}

	largest -= 1;

	// get smallest n-digit number
	x = 1;
	while(x<n)
	{
		smallest *= 10;
		x+=1;
	}
	
	let i : int;

	while(largest  >= smallest)
	{
		i = largest;
		while(i >= smallest)
		{
			if(i*largest < pal)
			{
				break;
			}

			if(IsPalindrome(i*largest) && i*largest > pal)
			{
				pal = i*largest;
			}

			i -=1;
		}

		largest -=1;
	}

	ret pal;
}

// test for solution
#[test]
fn testSolution()
{
	let result : int = solution(2);

	std::io::println(#fmt("Result: %d", result));
	assert result == 9009;

}