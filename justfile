set shell := ["bash", "-uc"]

# list all available recipes
@default:
	just --list --unsorted

# install dependencies
@install:
	npm install
alias i := install

# check if vsce is installed
@_doctor:
	if [ ! -f ./node_modules/.bin/vsce ]; then \
		printf "\033[1;35mvsce\033[0m not found -> \033[1;32minstalling...\033[0m\n"; \
		just install; \
	fi

# build extension
# @build: _doctor
# 	__start="$(date -u +%s.%N)"; \
# 	./node_modules/.bin/vsce package; \
# 	__end="$(date -u +%s.%N)"; \
# 	__elapsed=`bc <<< "( $__end - $__start )"`; \
# 	printf "\033[1;35mbuild\033[0m done in \033[1;32m%.2f\033[0m\033[32ms\033[0m\n" "0$__elapsed"
# alias b := build

# publish extension
@publish VERSION: _doctor
	case "{{VERSION}}" in \
		(major|minor|patch) ;; \
		(*) printf >&2 "\033[1;35mVERSION\033[0m must be '\033[1;32mmajor\033[0m', '\033[1;32mminor\033[0m' or '\033[1;32mpatch\033[0m' not '\033[1;31m{{VERSION}}\033[0m'\n"; exit 1 ;; \
	esac

	./node_modules/.bin/vsce publish "{{VERSION}}"; \
alias p := publish