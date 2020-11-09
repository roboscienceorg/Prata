
# prata (Distributed Communications)

### Goal
The goal of prata is to have a fast and easy distributed communication system that that someone could use without deep knowledge of sockets or ZeroMQ.

### Overview
```diff
- NEEDS UPDATE
```
prata will be a Publisher-Subscriber based project where the user will decide if they want to subscribe or publish to a specific channel. The handling of communications is done by a Master. This Master is the only connection information that the user needs to know. The Master handles the creation of new Channels if a Publisher requests a new one. The Master will handle the returning of a Publisher/Subscriber object that contains the Channel connection information back to the user. The user will not need to know the Channels connection information and may never even see it. ![Figure 1](Documentation/Images/flowchart1.jpg)
<br>
<br>
Publishers will only be able to send data to a Channel, they are also able to publish to multiple Channels.
Subscribers will only be able to receive data from a Channel, they are also able to receive from multiple Channels.
Channels will handle the storage of data independently from Publishers and Subscribers.

![Figure 2](Documentation/Images/flowchart2.jpg)


### Requirements/Building/Installation

#### Requirements
Cargo \
Maturin  \
Python 3.X \
pip \
Julia \
Windows/Linux \
Python Deps: \
    1. Pillow \
    2. NumPy


#### Steps for Building and Installing from Source
```diff

```
##### Python
1. Clone TALA through `git clone https://github.com/roboscienceorg/tala.git`.
2. cd into the `tala/TALA` folder.
3. Run install.bat (Windows) or install.sh (Linux) depending on your OS.
4. After building is finished, install the wheel file through `pip install Build/Wheels/[Wheel name]`. Depending on your system it may be `pip3`.
5. Import TALA into your python script to use TALA (See example below).

##### Julia
1. Clone TALA through `git clone https://github.com/roboscienceorg/tala.git`.
2. cd into the `tala/TALA` folder.
3. Run install.bat (Windows) or install.sh (Linux) depending on your OS.
4. In your Julia script import Pkg, activate the `Build/Julia/TALA.jl` folder  through `Pkg.activate([Path to Build/Julia/TALA.jl])` and then import TALA (See example below). The TALA.dll (Windows) or TALA.so (Linux) must be in the same directory as the Julia script.


### Using TALA
```diff

```
Simple script that will start TALA on IP 127.0.0.1 and port 25565, publish data
to a channel called "My Channel", then print that same data after listening from
that channel. Then terminating the TALA.
#### Python
```python
# Import the library
import TALA

# Connect to a host, this could be run running or one you plan on running
connection = TALA.connect("127.0.0.1", 25565)

# This is used to launch a host
connection.host()

'''
PUBLISHER
'''

# Create a publisher
publisher = connection.publisher()

# Connect to a channel, if this channel does not exist it will be created
publisher.connect("My Channel")

# Publish string to the channel
publisher.publish("My Channel", "Information")

'''
SUBSCRIBER
'''

# Create a subscriber
subscriber = connection.subscriber()

# Connect to a channel, if this channel does not exist it will be created
subscriber.connect("My Channel")

# Listen from the channel
data = subscriber.listen("My Channel")

print(data)


# Terminate the entire TALA network, stopping all channels.
connection.terminate()

'''
    publishers and susbcribers do not need to connect to a channel initially,
    publish and listen will connect to a channel if it is not alreay known.
'''
```

#### Julia
```Julia
# Import the TALA package
using Pkg
Pkg.activate("Path to Build/Julia/TALA.jl")
`Make sure TALA.dll/so is in the same directory as this file.`
using TALA

# Connect to a host, this could be run running or one you plan on running
connection = TALA.connect("127.0.0.1",25565)

# This is used to launch a host
TALA.host(connection)

`
PUBLISHER
`

# Create a publisher
publisher = TALA.publisher(connection)

# publish to a channel, if this channel does not exist it will be created
TALA.publish(publisher, "My Channel", "Information")

`
SUBSCRIBER
`

# Create a subscriber
subscriber = TALA.subscriber(connection)

# Listen from the channel
data = TALA.listen(subscriber, "My Channel")

println(data)

# Terminate the entire TALA network, stopping all channels.
TALA.terminate(connection)
```

### Core Developers
timadcock\
Ayden-Drabek\
ES3580\
ryan-shell\
bhllamoreaux

### Additional Help
jeffmcgough\
nibennett
