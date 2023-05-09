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
}

another_function() {
	local x=1
	local y=2
	local z

	z=$((x + y))
	echo "$z"
}

function some_other_function {
	cmd arg1 arg2 --flag1 --flag2
}
