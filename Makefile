IMGNAME = geth-exporter-rust
VERSION = $(shell grep '^version' Cargo.toml | sed -e 's/version = \"\(.*\)\"/\1/')

build:
	docker build . -t $(IMGNAME):latest

publish:
	docker tag $(IMGNAME):latest docker.pkg.github.com/ksato9700/geth-exporter-rust/$(IMGNAME):latest
	docker tag $(IMGNAME):latest docker.pkg.github.com/ksato9700/geth-exporter-rust/$(IMGNAME):$(VERSION)
	docker push docker.pkg.github.com/ksato9700/geth-exporter-rust/$(IMGNAME):latest
	docker push docker.pkg.github.com/ksato9700/geth-exporter-rust/$(IMGNAME):$(VERSION)

publish-ghcr:
	docker tag $(IMGNAME):latest ghcr.io/ksato9700/geth-exporter-rust/$(IMGNAME):latest
	docker tag $(IMGNAME):latest ghcr.io/ksato9700/geth-exporter-rust/$(IMGNAME):$(VERSION)
	docker push ghcr.io/ksato9700/geth-exporter-rust/$(IMGNAME):latest
	docker push ghcr.io/ksato9700/geth-exporter-rust/$(IMGNAME):$(VERSION)
