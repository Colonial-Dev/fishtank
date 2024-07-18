build:
	rm ./target/tankctl

	echo "#!/usr/bin/env fish"       >> ./target/tankctl
	cat ./src/util/*                 >> ./target/tankctl
	cat ./src/main.fish              >> ./target/tankctl
	
	chmod +x ./target/tankctl

run:
	@./target/tankctl