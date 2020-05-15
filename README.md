# grep-rs  
A grep-like tool written in rust  
  
#### Currently working features  
Searching through files  
Searching through standard input  
Excluding patterns  
Printing all lines before the first instance of the pattern  
Printing all lines after the first instance of the pattern  
#### Examples  
##### Searching through stdin  
ls | grep-rs basic_file_1  
output: basic_file_1.txt  
  
##### Searching through files  
grep-rs text basic_file_1.txt  
output: The text states .....  
  
##### Excluding patterns  
ls | grep-rs -e basic_file_1  
output:  
basic_file_2.txt  
basic_file_3.txt  
basic_file_4.txt  
##### Printing all lines before the first instance of the pattern  
ls | grep-rs -I basic_file_3  
output:  
basic_file_1.txt  
basic_file_2.txt  
basic_file_3.txt  
##### Printing all lines after the first instance of the pattern  
ls | grep-rs -i basic_file_3  
output:  
basic_file_3.txt  
basic_file_4.txt  