Improved cd to be used in terminal. See zoxide for a similar build.

Three parts: 
The model which returns all the subdirectories and their path.
The view which is a command line UI.
The parser which parses the inputs and starts the process.

If the folder does not exist it will recursivly search down to se if the folder exists there.
It will by default search all the way down and if there are multiple folders with that name it will let you chose
for example :
[1] path/japan
[2] otherpath/japan
[3] lastpath/japan

You may add a flag to tell how deep max it will search, for example 'cd afolder -depth=2'.
You can also add a flag to tell it to also search up, for example 'cd afolder -all'.

write something in the lines of:
'cdd path_you_want_to_go_to'

cdd will be an alias for a pipe which will look something like:
'improved_cd path_you_want_to_go_to | cd'
(improved_cd is the name of the executable)

the error handling will take place in the program and will exit the process
if something goes wrong.