set windows-powershell := true


default:
	cargo run --release -- tests/adn-10000-10000.test

test:
	cargo test --workspace --all-features --release

fix:
	cargo clippy --workspace --all-features --fix --allow-dirty --allow-staged

fmt:
	cargo fmt --all

prettier:
	# install on Debian: sudo snap install node --classic
	# npx is the command to run npm package, node is the runtime
	npx prettier -w .


git-cache:
	git rm -rf --cached .
	git add .


expand:
	cargo expand


java:
	javac java_implementation/RecherchePLSSC.java
	java -cp java_implementation RecherchePLSSC tests/adn-10000-10000.test