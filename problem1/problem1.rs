use std;

//////////////////////////////////////////////////////////
//
// Problem:
// Find the sum of all the multiples of 3 or 5 below 1000.
//
//////////////////////////////////////////////////////////

fn main()
{
	std::io::println(#fmt("Solution: %d",solution(3,5,1,1000)));
}

// my first solution : O(n)
fn solution(x: int, y: int, from: int, to: int) -> int
{
	let sum = 0;
	let counter = from;

	while counter < to
	{
		if (counter % x == 0 || counter % y == 0)
		{
			sum += counter;
		}

		counter += 1;
	}

	ret sum;
}

// test for solution
#[test]
fn testSolution()
{
	let result: int = solution(3,5,1,10);

	assert result == 23;

}