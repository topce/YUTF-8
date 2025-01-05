# YUTF-8
What is YUSCII to ASCII , that is YUTF-8 to UTF-8 ;-)

YUTF-8
Swap YUSCII chars in UTF-8
https://en.wikipedia.org/wiki/YUSCII

So as UTF-8 is compatible to ASCII 
YUTF-8 is compatible with YUSCII

YUTF-8 is my charachter encoding standard that similar like  UTF-8
that swap YUSCII charachters.

It could be used to in 3 variation to encode (use BOM  chrs for this?)  
Serbian Latin (Slovenian and Croatian as well),  
Serbian Cyrillic,
Macedonin Cyrillic.

Similar could be done for  KOI 
https://en.wikipedia.org/wiki/KOI-7 
Russian and Ukrainian 
but it is out of scope of this project.

# Why maybe ,sombody would use YUTF-8

to have less network traffic for Serbian texts  without compression,
also interesting for LLM maybe it will use less tokens for prompting in for example Serbian Launguage. 
it loook like all mainstream LLM are large english language models LELM ;-)

# TODO
implment rust linrary (WASM) UTF-8 <-> YUTF-8 libs similar like https://github.com/topce/cirilica
to make easy to use and experiment with YUTF-8 new encoding standard. 


