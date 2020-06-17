# SDL2 Tutorial

## Setting up SDL2 dll in windows

To run the examples in each part it is important to install the SDL2 libraries in your
system. In Linux and iOS that can be accomplished using the corresponding package manager.

However, to use SDL2 in windows one how to download the 
[compiled files](https://www.libsdl.org/download-2.0.php) and place them
in a location where cargo and the executable file can find them. The dll files have to 
be seen by the executable the lib files have to be seen by cargo
The next steps can be used to create a folder with the files available to cargo:

1. Create "dll" folder in HOMEPATH.
2. Download and extract dll and lib files in folder. Place in this folder the files 
corresponding to your machine or architecture.
3. Add "dll" folder to PATH. By adding this folder to the PATH in your machine, the executable
file will find the library file.
4. Add a build.rs file. The build.rs file will indicate where cargo has to look for the 
lib files. Each part in this repository has a build.rs file that points to HOMEPATH\dll. 
