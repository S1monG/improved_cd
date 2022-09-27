Improved cd to be used in terminal. See zoxide for a similar build.

It will by default search all the way down and if there are multiple folders with that name it will let you chose
for example if you search for a directory 'japan' :
[1] path/japan  
[2] otherpath/japan  
[3] lastpath/japan  
  
You may add a flag to control the max depth it will search, for example 'cdd afolder --max-depth 2'.  
In the case where there are more than one directory with the name you are searching for you  
can add a flag to always choose the closest, for example 'cdd afolder -c'.  
You can also add a flag to tell it to search from the root, for example 'cdd afolder -r'.  
  
You need to add a script in your $PROFILE file. This may look different depending on what terminal  
you are using. This is an example of how it may look in PowerShell  
![Alt text](/images/improved_cd_screenshot.jpg "Screenshot of my powershell script")