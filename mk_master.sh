#!/bin/bash

base="cd latex && latexmk -pdfxe matfmaster-primer-lat && mv matfmaster-primer-lat.pdf ../"

if [ $1 != "-nc" ]
then
   base+=" && latexmk -c matfmaster-primer-lat"
fi

eval $base
