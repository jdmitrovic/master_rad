#!/bin/bash

(cd latex && latexmk -pdfxe -output-directory="../" matfmaster-primer-lat && cd ../ && latexmk -c latex/matfmaster-primer-lat)
