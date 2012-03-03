use std;
//////////////////////////////////////////////////////////
//
// Problem 6:
// Find the difference between the sum of the squares of 
// the first one hundred natural numbers and the square 
// of the sum
//
//////////////////////////////////////////////////////////

fn main()
{
	std::io::println(#fmt("Solution: %d",solution(100)));
}

fn solution(x: int) -> int
{	
	let dif : int = 0;
	let sumsqr : int = 0;
	let sum : int = 0;
	let i : int = 1;

	// sum squares
	while(i <= x)
	{
		sumsqr += i * i;
		i +=1;
	}

	// sum
	i = 1;
	while( i <= x)
	{
		sum += i;
		i+=1;
	}

	dif = (sum * sum) - sumsqr;

	ret dif;
}

// test for solution
#[test]
fn testSolution()
{
	let result: int = solution(10);

	//std::io::println(#fmt("Result: %d", result));
	assert result == 2640;

}