# Show
[ grep,tail,cat ] like cli tool written in rust.
Only one release as of now which does very basic function,code has been refactored where other functionalities can be added without having to change everything so will be adding more features soon.

Since there is only **one release** so i do not recommend expecting many things and it **supports linux only** as of now. This is very basic ,while i was trying one of the example of [Rust's official book ](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)i got the thought of writing my own tool which does the functions similar to the linux commands such as grep,tail and cat respectively.

**For installation , download this zip,extract the folder , cd into the downloaded file and run sudo ./install.sh .**

_I recommend copy pasting the commands given below and install directly from terminal._


`wget https://github.com/sirimhrzn/show/releases/download/0.1.0/show-v0.1.0.tar.gz`
> _Download the zip directly from terminal or you can manually download from the [release page](https://github.com/sirimhrzn/show/releases/tag/0.1.0)_

`tar -xzf show-v0.1.0.tar.gz` 

> _Optional if manually extracted_

`cd show-v0.1.0`
> _Change directory to extracted folder_

`sudo ./install.sh`
>_Install the application_

Feel free to use it. As of now it only supports

`show [QUERY] -F [FILE_PATH]`

 _I made a release of this project with the mind of learning the basics of shell language , experience releasing builds with bash scripts which installs the application._
