prog :=xnixperms

debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

build:
	cargo build $(release)

install:
	cp target/$(target)/wallpaper-ctl ~/bin/wallpaper-ctl
	cp target/$(target)/wallpaper-manager ~/bin/wallpaper-manager
	cp target/$(target)/wallpaper-engine ~/bin/wallpaper-engine
	echo "[Desktop Entry]\nType=Application\nName=Wallpaper-manager\nComment=Wallpaper manager\nExec=~/bin/wallpaper-manager %U\nIcon=wallpaper-manager\nTerminal=false" > ~/.local/share/applications/wallpaper-manager.desktop

all: build install

uninstall:
	rm ~/bin/wallpaper-ctl
	rm ~/bin/wallpaper-gui
	rm ~/bin/wallpaper-engine
	rm ~/.local/share/applications/wallpaper-manager.desktop

help:
	@echo "usage: make $(prog) [debug=1]"