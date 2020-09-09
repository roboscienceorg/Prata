#!/usr/bin/env python3
import time
import zmq
import random
import sys


# Connect to the master port 
print("Client socket waiting for connection...")

# Set desired master port number
masterPort = "5000"

#############################################################
# Function to request port number from master server
#############################################################

def getPort(nodeType):
		# Set up request socket
		context = zmq.Context()
		socket = context.socket(zmq.REQ)
		socket.connect("tcp://localhost:%s" % masterPort)

		# Request to subscribe to channel 1
		channel = 1
		print("Sending request...")
		socket.send_string("%d %s" % (channel, nodeType))

		#  Get the backend port number reply from the master
		port = socket.recv_string()
		print("Received Backend Port Number: ", port)

		return port


# Start subscribing to the received port here
# Call the getPort function here
socketPort = getPort("SUBSCRIBER")
context = zmq.Context()
subscriber = context.socket(zmq.SUB)
subscriber.connect("tcp://localhost:%s" % socketPort)

# Subscribe to channel 1
topic = "1"
channel = bytes(topic,"ascii")
subscriber.setsockopt(zmq.SUBSCRIBE, channel)

print("Socket connecting to Forwarder backend port...")

while True:
	# Keep receiving messages from publisher
	message = subscriber.recv_string()
	channel, messagedata = message.split()
	print("Message Received: ", channel, messagedata)
