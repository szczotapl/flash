all: install

install:
	bash install.sh
uninstall:
	sudo rm -f /usr/bin/flash
	sudo rm -rf $(HOME)/.flash/

.PHONY: install uninstall