use std;
//////////////////////////////////////////////////////////
//
// Problem 5:
// What is the smallest positive number that is evenly 
// divisible by all of the numbers from 1 to 20?
//
//////////////////////////////////////////////////////////

fn main()
{
	std::io::println(#fmt("Solution: %d",solution(20)));
}

fn solution(x: int) -> int
{	
	let found : bool = false;
	let number : int = 0;

	while ( !found )
	{
		let i : int = x;
		found = true;
		number += x;

		while ( i > 0)
		{
			if ( number % i != 0)
			{
				found = false;
				break;
			}

			i -= 1;
		}
	}
	
	ret number;
}

// test for solution
#[test]
fn testSolution()
{
	let result: int = solution(10);

	//std::io::println(#fmt("Result: %d", result));
	assert result == 2520;

}