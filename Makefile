all: check vdp cargo

check:
	@if [ ! -f ./src/vdp/userspace-vdp-gl/README.md ]; then echo "Error: no source tree in ./src/vdp/userspace-vdp."; echo "Maybe you forgot to run: git submodule update --init"; echo; exit 1; fi

vdp:
ifeq ($(PLATFORM),mac)
	echo "ON MAC!"
	EXTRA_FLAGS=-Wc++11-narrowing $(MAKE) -C src/vdp
else
	$(MAKE) -C src/vdp
endif
	cp src/vdp/*.so firmware/

cargo:
	FORCE=1 cargo build -r
	cp ./target/release/fab-agon-emulator .

vdp-clean:
	$(MAKE) -C src/vdp clean

cargo-clean:
	cargo clean

clean: vdp-clean cargo-clean

depends:
	$(MAKE) -C src/vdp depends
