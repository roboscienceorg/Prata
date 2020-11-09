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
