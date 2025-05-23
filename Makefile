CARGO := cargo
DOCKER := docker
OSV_SCANNER := osv-scanner

CARGO_REGISTRY ?= ${HOME}/.cargo/registry
DOCKER_IMAGE ?= buildenv
TAG ?= base
SBOM_FILE ?= sbom.spdx.json

pkg:
	@CMD="make generate-sbom" make docker-exec
	@CMD="addlicense"

generate-sbom:
	$(CARGO) sbom > ${SBOM_FILE}
	$(CARGO) cyclonedx -f json -a
	# can fail
	$(OSV_SCANNER) scan -r . || true

fetch:
	$(CARGO) fetch

update:
	$(CARGO) update

build:
	$(CARGO) build --release

addlicense:
	$(DOCKER) run --rm -it -v ${PWD}:/src ghcr.io/google/addlicense:latest \
		-c "Kensuke Saito" \
		-l GPL-2.0-only \
		-s=only \
		$(shell find src -type f -name "*.rs")

docker-exec:
	$(DOCKER) run --rm -it \
		-v $(shell pwd):/app \
		-v ${CARGO_REGISTRY}:/usr/local/cargo/registry \
		${DOCKER_IMAGE}:${TAG} \
		bash -c "$(CMD)"
