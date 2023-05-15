#ifndef TMP_HPP
# define TMP_HPP

#include "utils.hpp"
#include <iostream>

using namespace module;

class Foo
{
public:
	Foo(void);
	Foo(const Foo &src);
	~Foo(void);

	Foo		&operator=(const Foo &rhs);

private:
	int		_x;
};

int	foo(int ac, char **av, const std::string &tmp)
{
	static long				sl;
	std::string				s = "Hello Wordl!\n";
	std::string::iterator	y = s.begin();
	int						x = 1234567890;

	x = x + x;
	x = x - x;
	x = x * x;
	x = x / x;
	x += x;
	x -= x;
	x *= x;
	x /= x;
	x++; ++x;
	x--; --x;
	y++; ++y;
	y--; --y;

	av[0] = NULL;
	av->hey;
	av.hey;
	y->begin();
	y->hey;
	(*(*y)).begin();

	x = &sl;
	x = *sl;

	if ((x == x || x != x) && x == x && sizeof(x))
	while (((((true)))))
		x = false;

	foo(x, NULL, NULL);

	)
	// comment
	/*
	multi line comment
	*/

	return (0);
}

#endif
