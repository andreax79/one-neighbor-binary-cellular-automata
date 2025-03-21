SHELL=/bin/bash -e

help:
	@echo - make black ------ Format code
	@echo - make isort ------ Sort imports
	@echo - make clean ------ Clean virtual environment
	@echo - make venv ------- Create virtual environment

.PHONY: isort
codespell:
	@codespell -w *.py

.PHONY: isort
isort:
	@isort --profile black *.py

.PHONY: black
black: isort
	@black -S *.py

.PHONY: clean
clean:
	-rm -rf build dist
	-rm -rf *.egg-info
	-rm -rf bin lib lib64 share include pyvenv.cfg

.PHONY: venv
venv:
	python3 -m venv . || python3 -m virtualenv .
	. bin/activate; pip install -Ur requirements.txt
	. bin/activate; pip install -Ur requirements-dev.txt
