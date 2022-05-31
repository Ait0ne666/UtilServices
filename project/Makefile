SHELL=cmd.exe
UTS_BINARY=utsApp


run_build:	
	@echo Stopping docker images (if running...)
	docker-compose down
	@echo Building (when required) and starting docker images...
	docker-compose -p uts up --build -d
	@echo Docker images built and started!




build_cli:
	@echo Building cli binary...
	chdir ..\uts\uts-cli  && cargo build --release --target x86_64-unknown-linux-musl
	@echo Done!

down:
	@echo Stopping docker compose...
	docker-compose -p uts down --remove-orphans
	@echo Done!