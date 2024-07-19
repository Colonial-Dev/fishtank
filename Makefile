format:
	for file in $$(find src/ -name "*.fish"); do \
		sed -i '$$a\' $$file; \
		fish_indent -w $$file;   \
	done

build:
	rm -f ./target/tankctl

	echo "#!/usr/bin/env fish"       >> ./target/tankctl
	cat ./src/util/*                 >> ./target/tankctl
	cat ./src/main.fish              >> ./target/tankctl
	
	chmod +x ./target/tankctl

run:
	@./target/tankctl