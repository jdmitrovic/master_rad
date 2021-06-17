#!/bin/bash

(cd latex && latexmk -pdfxe matfmaster-primer-lat && latexmk -c matfmaster-primer-lat && mv matfmaster-primer-lat.pdf ../)
