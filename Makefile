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
	$(OSV_SCANNER) scan -r .

fetch:
	$(CARGO) fetch

update:
	$(CARGO) update

docker-exec:
	$(DOCKER) run --rm -it \
		-v $(shell pwd):/app \
		-v ${CARGO_REGISTRY}:/usr/local/cargo/registry \
		${DOCKER_IMAGE}:${TAG} \
		bash -c "$(CMD)"

addlicense:
	$(DOCKER) run --rm -it -v ${PWD}:/src ghcr.io/google/addlicense:latest \
		-c "Kensuke Saito" \
		-l GPLv2 \
		-s=only \
		$(shell find src -type f -name "*.rs")
