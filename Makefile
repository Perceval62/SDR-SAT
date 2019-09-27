COMPILER = cargo
ARGS = build

debug: src/*
	$(COMPILER) $(ARGS)

release: src/*
	$(COMPILER) $(ARGS) --release

doc: Doc/*
	./Doc/buildDocument.sh