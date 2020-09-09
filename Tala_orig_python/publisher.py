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

	# Request to talk on channel 1
	channel = 1
	print (channel, nodeType)
	print("Sending request...")
	socket.send_string("%d %s" % (channel, nodeType))

	#  Get the frontend port number reply from the master
	port = socket.recv_string()
	print("Received Frontend Port Number: ", port)

	return port

# Start publishing on the received port here
# Call the getPort function here
socketPort = getPort("PUBLISHER")
context = zmq.Context()
publisher = context.socket(zmq.PUB)
publisher.connect("tcp://localhost:%s" % socketPort)

print("Socket connecting to Forwader frontend port....")

while True:
	# Talk on channel 1
	channel = 1
	message = ("%s" % socketPort)
	# Send channel and port numbers as message to subscribers
	publisher.send(bytes("%d %s" % (channel, message),'ascii'))
	print("Sent:","%s %s" % (channel, message))
	time.sleep(1)