all:
	cargo build
package:
	make clean
	cargo package
clean:
	cargo clean
	rm -f clips_core_source_630/core/*.o clips_core_source_630/core/*.a clips_core_source_630/core/*~ clips_core_source_630/core/clips

