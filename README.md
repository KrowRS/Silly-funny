# GUIDE
## Setting up the Extension
Hello gamers CUM is actually super easy to use but I'm lazy and don't want to make it have a GUI or something so please follow this guide

If you are using Firefox or some version of Firefox you're super sexy and this will be easy for you, if you're using OperaGX or Google chrome please reevaluate and also this won't work for you yet I might figure it out for you later.

Go get this extension for your browser https://addons.mozilla.org/en-US/firefox/addon/send-tab-url/

Once you have the extension please go to the manage section and under options you will have the ability to change the name of the location you would like to send it to and the target URL. For our purposes you will use these settings

Name: Localhost

Target URL: http://localhost:3000/fuml?url={URL}&title={TITLE}

![firefox_xAB0Wn6m2w](https://github.com/KrowRS/C.U.M.-CLAIMER-OF-UTUBE-MEDIA-/assets/80224801/692d6c9b-9eed-429d-adf2-bcc43e5ff322)

REMEMBER TO PRESS THE SAVE BUTTON 
This is the extension we will use to download youtube videos by sending the data from the browser to the cum.exe by pressing "send to localhost" whenever interacting with the extension on a Youtube page.

## Setting up the Application
Now that you have the extension working you will have a .rar from the release page containing 3 files: "cum.exe", "directories.json", and "yt-dlp.exe".

Cum.exe is the file that will receive information from the extension it MUST be running for this to work.

yt-dlp.exe is a file which is given arguements by the cum.exe file in order to actually install the youtube video at the target location.

Directories.json contains the configuration data and MUST be in the same directory as "cum.exe", by default it will contain:

```json
{
    "ytdlp":"C:\\yt-dlp.exe",
    "download":"D:\\BULLSHIT"
}
```


In order to have cum.exe work we need to tell it where your yt-dlp.exe file is located, if you placed it within your downloads folder for example you might need to replace the contents to the right of "ytdlp": with something like "C:\\Users\\Name\\Downloads\\yt-dlp.exe". 

"Download" on the other hand indicates the directory you would like to have your Youtube videos downloaded to, if you wanted them to be put into a folder in your videos folder you might do something like "C:\\Users\\Name\\Videos\\cumvideos"

So in your case you might need to have the directory.json look like this

```json
{
    "ytdlp":"C:\\Users\\Name\\Downloads\\yt-dlp.exe",
    "download":"C:\\Users\\Name\\Videos\\cumvideos"
}
```

*IT IS IMPORTANT WHEN REPLACING THE FILE LOCATIONS TO ENSURE THAT YOU ARE USING DOUBLE BACKSLASHES.*

To reiterate, Cum.exe and Directories.json must be in the same place as each other, but yt-dlp can be placed anywhere since you are telling cum.exe where it is placed in directories.json.

## Running it
Now that everything is configured to work on your machine you can run cum.exe (it should not be obvious that it is running but can be tested by going to http://localhost:3000/ in a browser) 

Visit a youtube video and interact with the extension and click "send to localhost", assuming everything worked correctly you should have a blank terminal appear that indicates the video is being downloaded (DO NOT CLOSE THIS TERMINAL, IT WILL AUTOMATICALLY CLOSE ONCE THE VIDEO IS DOWNLOADED). 

Currently the videos are set to install in a suboptimal resolution and fps in order to ensure you will not nuke your drive with videos but I may add customizable args for yt-dlp in the future.

## EXTRA THING 
If you want cum.exe to run on startup you can make a shortcut to cum.exe and place it within shell:startup in a file browser which can also be accessed through it's appdata location (AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup)
![explorer_5xY0R4Pl5W](https://github.com/KrowRS/C.U.M.-CLAIMER-OF-UTUBE-MEDIA-/assets/80224801/e09068fd-cf32-4a37-9ed1-adeb5f4bdba1)

