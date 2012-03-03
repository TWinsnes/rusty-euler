use std;
//////////////////////////////////////////////////////////
//
// Problem 372:
// Let R(M, N) be the number of lattice points (x, y) 
// which satisfy M<x≤N, M<y≤N and is odd.
//
//////////////////////////////////////////////////////////

fn main()
{
	std::io::println(#fmt("Solution: %d",R(2000000, 1000000000)));
}

// working, but too slow O(((n-m)^2)/2)
fn R(m: int, n : int) -> int
{
	let count : int = 0;
	let y : int = m + 1;

	while(y <= n)
	{

		let x : int = m + 1;
		while ( x <= y)
		{
			let ysqr : int = y*y;
			let xsqr : int = x*x;
			let div : int = ysqr / xsqr;

			if(div % 2 != 0)
			{
				count += 1;
			}
			x += 1;
		}

		y += 1;
	}

	ret count;
}

// test for solution
#[test]
fn testSolution()
{
	assert R(0, 100) == 3019;
	assert R(100, 10000) == 29750422;

}