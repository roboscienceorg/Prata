```diff
# prata (Distributed Communications)

### Goal
The goal of prata is to have a fast and easy distributed communication system that that someone could use without deep knowledge of sockets or ZeroMQ.

### Overview
- NEEDS UPDATE
prata will be a Publisher-Subscriber based project where the user will decide if they want to subscribe or publish to a specific channel. The handling of communications is done by a Master. This Master is the only connection information that the user needs to know. The Master handles the creation of new Channels if a Publisher requests a new one. The Master will handle the returning of a Publisher/Subscriber object that contains the Channel connection information back to the user. The user will not need to know the Channels connection information and may never even see it. ![Figure 1](Documentation/Images/flowchart1.jpg)
<br>
<br>
Publishers will only be able to send data to a Channel, they are also able to publish to multiple Channels.
Subscribers will only be able to receive data from a Channel, they are also able to receive from multiple Channels.
Channels will handle the storage of data independently from Publishers and Subscribers.

![Figure 2](Documentation/Images/flowchart2.jpg)


### Requirements/Building/Installation

##### Requirements
Cargo \
Maturin  \
Python 3.X \
pip \
Julia \
Python Deps:
    1. Pillow \
    2. NumPy \
Windows/Linux \

##### Steps for Building and Installing from Source
- NEEDS UPDATE
1. On Windows run install.bat, on Linux run install.sh they are located in the tala/Rust folder. This should take some time since it is installing and building everything. They will also tell you if you are missing any requirements.
2. Go to Build for direct libraries, Build/Wheel for Python wheels. <br>
3. If you want to use the direct libraries move the TALA.pyd into the location of the Python script and import TALA as usual. If you want to install TALA go into the Build/Wheels folder and run `pip install [Wheel Name].whl`. The wheel name will changed based on your system and Python Version. <br>
4. For Julia, go into the Build/Julia folder, launch the Julia REPL press `]`, then type `activate TALA.jl`, backspace until Julia is seen again and do `using TALA` then TALA should be installed.


### Using TALA
- NEEDS UPDATE
[How to use this should come soon]

### Core Developers
timadcock
Ayden-Drabek
ES3580
ryan-shell
bhllamoreaux

### Additional Help
jeffmcgough
nibennett
```
