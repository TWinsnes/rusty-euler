use std;
//////////////////////////////////////////////////////////
//
// Problem 7:
// What is the 10 001st prime number?
//
//////////////////////////////////////////////////////////

fn main()
{
	std::io::println(#fmt("Solution: %d",solution(10001)));
}

fn solution(x: int) -> int
{	
	let primes : [int] = [2];
	let i : int = 1;

	while ( vec::len::<int>(primes) as int < x)
	{
		let prime : bool = true;
		i += 2;
		for x in primes
		{
			if(i % x == 0)
			{
				prime = false;
				break;
			}
		}

		if(prime)
		{
			vec::push::<int>(primes, i);
		}
	}
		
	ret primes[vec::len::<int>(primes)-1u];
}

// test for solution
#[test]
fn testSolution()
{
	let result: int = solution(6);

	//std::io::println(#fmt("Result: %d", result));
	assert result == 13;

}