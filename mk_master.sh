#!/bin/bash

eval "cd latex && latexmk -pdfxe MasterRadJovanDmitrovic && mv MasterRadJovanDmitrovic.pdf ../"

if [[ $1 != "-nc" ]]
then
   eval "latexmk -c MasterRadJovanDmitrovic"
fi

