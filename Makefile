# Auto-format all source files (using fish_indent)
# and add any missing trailing newlines.
format:
	for file in $$(find src/ -name "*.fish"); do \
		fish_indent -w $$file; \
	done

lint:
	@set -e
	@for file in $$(find src/ -name "*.fish"); do \
		fish_indent -c $$file; \
	done

build:
	rm -f ./target/tankctl

	echo "#!/usr/bin/env fish"       >> ./target/tankctl
	cat ./src/util/*                 >> ./target/tankctl
	cat ./src/main.fish              >> ./target/tankctl
	
	chmod +x ./target/tankctl

run:
	@./target/tankctl