use std;
//////////////////////////////////////////////////////////
//
// Problem:
// By considering the terms in the Fibonacci sequence 
// whose values do not exceed four million, find the sum 
// of the even-valued terms
//
//////////////////////////////////////////////////////////

fn main()
{
	std::io::println(#fmt("Solution: %d",solution(4000000)));
}

// my first solution
fn solution(x: int) -> int
{
	let sum = 0;
	let previous : int = 1;
	let current : int = 2;

	while current < x
	{
		if current % 2 == 0
		{
			sum += current;
		}

		let newValue : int = previous + current;
		previous = current;
		current = newValue;
	}


	ret sum;
}

// test for solution
#[test]
fn testSolution()
{
	let result: int = solution(100);

	assert result == 44;

}