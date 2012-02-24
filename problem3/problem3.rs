use std;
//////////////////////////////////////////////////////////
//
// Problem 3:
// What is the largest prime factor of the number 
// 600851475143 ?
//
//////////////////////////////////////////////////////////

fn main()
{
	std::io::println(#fmt("Solution: %d",solution(600851475143)));
}

//////////////////////////////////////////////////////////
//
// Idea behind solution:
// If you start at the smallest prime (x = 2), divide the
// remainder (y) by x if x % y == 0 and iterate by 1 every 
// time x % y != 0 you will find that the only numbers 
// that x % y == 0 are when x is a prime.
//
// This is because every number that is not a prime must be 
// dividable a smaller prime and already removed.
//
// Worst case performance: O(n-1) when input is a prime
//
//////////////////////////////////////////////////////////
fn solution(x: int) -> int
{	
	let number : int = x;
	let factor : int = 2;

	while (factor < number)
	{
		while (number % factor ==0)
		{
			number /= factor;

			std::io::println(#fmt("Prime: %d\tRemander: %d", factor, number))
		}

		factor+=1;
	}

	ret factor;
}

// test for solution
#[test]
fn testSolution()
{
	let result: int = solution(13195);

	//std::io::println(#fmt("Result: %d", result));
	assert result == 29;

}