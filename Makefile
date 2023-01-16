PROGRAM_NAME := pg2_test_rust

all: clean build

clean:
		rm -f bin/*
		# rm -f target/mipsel-unknown-linux-gnu/release/${PROGRAM_NAME}
		# cargo clean

build:
	cargo build --release --package ${PROGRAM_NAME} --bin ${PROGRAM_NAME}

mips:
	cargo build \
	--release \
	--package ${PROGRAM_NAME} --bin ${PROGRAM_NAME} \
	--target mipsel-unknown-linux-gnu
	cp target/mipsel-unknown-linux-gnu/release/${PROGRAM_NAME} \
	bin/${PROGRAM_NAME}.gcw

squash:
	mksquashfs bin/${PROGRAM_NAME}.gcw \
 	gcw0/${PROGRAM_NAME}.png \
 	gcw0/default.gcw0.desktop \
 	bin/${PROGRAM_NAME}.opk \
 	-all-root -no-xattrs -noappend -no-exports

opk: clean mips squash

deploy: opk
	scp bin/${PROGRAM_NAME}.opk root@10.1.1.2:/media/data/apps
