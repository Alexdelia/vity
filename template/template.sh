#!/bin/bash

alias some='command'
alias another="command"
unalias some

function some_function() {
	local var="some value"
	typeset -a array

	array=(1 2 3 4 5)
	array[0]=1

	echo "$var" "${array[@]}"

	for i in "${array[@]}"; do
		echo "$i"
	done

	while [[ "$var" -lt 10 ]]; do
		echo "$var"
		var=$((var + 1))
	done
}

another_function() {
	local x=1
	local y=2
	local z

	z=$((x + y))

	if [[ "$z" -eq 3 ]]; then
		echo "yes"
	elif [[ "$z" -le 2 ]]; then
		echo "no"
	else
		echo "indeed"
	fi
}

function some_other_function {
	cmd arg1 arg2 --flag1 --flag2
}
