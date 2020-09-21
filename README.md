# TALA
## Distributed Communications

### Goal
The goal of TALA is to have a fast and easy distributed communication system that that someone could use without deep knowledge of sockets or ZeroMQ.

### Overview
TALA will be a Publisher-Subscriber based project where the user will decide if they want to subscribe or publish to a specific channel. The handling of communications is done by a Master. This Master is the only connection information that the user needs to know. The Master handles the creation of new Channels if a Publisher requests a new one. The Master will handle the returning of a Publisher/Subscriber object that contains the Channel connection information back to the user. The user will not need to know the Channels connection information and may never even see it. ![Figure 1](Documentation/Images/flowchart1.jpg)
<br>
<br>
Publishers will only be able to send data to a Channel, they are also able to publish to multiple Channels.
Subscribers will only be able to receive data from a Channel, they are also able to receive from multiple Channels.
Channels will handle the storage of data independently from Publishers and Subscribers.

![Figure 2](Documentation/Images/flowchart2.jpg)
