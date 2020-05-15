# rgrep  
A grep implementation in rust  
  
#### Currently working features  
Searching through files  
Searching through standard input  
Excluding patterns  
Printing all lines before the first instance of the pattern  
Printing all lines after the first instance of the pattern  
#### Examples  
##### Searching through stdin  
ls | rgrep basic_file_1  
output: basic_file_1.txt  
  
##### Searching through files  
rgrep text basic_file_1.txt  
output: The text states .....  
  
##### Excluding patterns  
ls | rgrep -e basic_file_1  
output:  
basic_file_2.txt  
basic_file_3.txt  
basic_file_4.txt  
##### Printing all lines before the first instance of the pattern  
ls | rgrep -I basic_file_3  
output:  
basic_file_1.txt  
basic_file_2.txt  
basic_file_3.txt  
##### Printing all lines after the first instance of the pattern  
ls | rgrep -i basic_file_3  
output:  
basic_file_3.txt  
basic_file_4.txt  