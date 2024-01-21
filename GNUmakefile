.PHONY:- help
-:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?##/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo
more:## 	more help
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/	/'
	#$(MAKE) -f Makefile help

-include Makefile

gnostr-test:
	@echo $(shell which gnostr)
	@echo $(shell which gnostr-weeble)
	@echo $(shell which gnostr-post-event)
	gnostr --sec $(shell gnostr-sha256 $(shell gnostr-weeble)) | gnostr-post-event ws://0.0.0.0:8080
gnostr-test-proxy:
	@echo $(shell which gnostr)
	@echo $(shell which gnostr-weeble)
	@echo $(shell which gnostr-post-event)
	gnostr --sec $(shell gnostr-sha256 $(shell gnostr-weeble)) | gnostr-post-event ws://0.0.0.0:6102

-include cargo.mk
