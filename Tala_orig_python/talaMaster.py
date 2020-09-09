#!/usr/bin/env python3

################################################################################
# Author(s): Aladi Akoh, ............
# Professor: Dr. Jeff McGough
# Usage: Start up talaMaster on a terminal using the command:
# ./talaMaster.py 
# Start up another terminal and run the client program using the command: 
# ./name_of_client.py
################################################################################

import time
import zmq
import threading as thr
from multiprocessing import Process

# Imports from TALA GUI CODE
import PySimpleGUI as sg
import numpy as np
import networkx as nx
import pandas as pd
import matplotlib
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg
import matplotlib.pyplot as plt


################################################################################
# Function to decode received message from client (pub/sub)
################################################################################
def split_message(message):
	channel, comm_type = message.split()
	return channel, comm_type

################################################################################
# Function to send correct port number to client(pub/sub)
################################################################################
def pack(comm_type,pub_port, sub_port, socket):

	# If client is a publisher, send the Forwarder subcriber port number
	# If client is a subscriber, send the Forwarder publisher port number
		if (comm_type == "PUBLISHER"):
			print("Sending port number (", sub_port ,")to client")
			socket.send_string(sub_port)

		elif (comm_type == "SUBSCRIBER"):
			print("Sending port number (", pub_port, ")to client")
			socket.send_string(pub_port)


################################################################################
# Function to fire up a forwader based on client's request
################################################################################
def forwader(sub_port, pub_port):

		# This code will actually run in a separate process
		# And separate context.
				try:
					context = zmq.Context(1)
					# Socket facing clients
					frontend = context.socket(zmq.SUB)
					frontend.bind("tcp://*:%s" % sub_port)
					frontend.setsockopt_string(zmq.SUBSCRIBE, "")
					print("Forwader: Frontend subscriber socket initializing...")

					# Socket facing services
					backend = context.socket(zmq.PUB)
					backend.bind("tcp://*:%s" % pub_port)
					print("Forwarder: Backend publisher socket initializing...")
					zmq.device(zmq.FORWARDER, frontend, backend)
				except Exception as e:
					print (e)
					print ("bringing down zmq device")

################################################################################
# Sever Function here
################################################################################
def serverFunction(mode):

	global server_run, data, channel_list, pub_port, sub_port

	# Socket to connect to clients
	print("Collecting updates from clients...")
	context = zmq.Context()
	master_reply = context.socket(zmq.REP)
	# Get the master port number from the main function & bind
	master_reply.bind("tcp://*:" + str(data[0]))
	# Initialize the port number for clients(pub/sub)
	port = data[1]

	while (server_run):
		# Wait for request from client(pub/sub) node
		try:
			message = master_reply.recv_string(flags = zmq.NOBLOCK)
		except zmq.Again as e:
			continue
		# Call the split_message function here to decode message from client
		channel, key = split_message(message)
		channel_name = channel
		comm_type = key

		# Check/decide what to do if channel exists in dictionary or not
		# If channel exists, do not create new port numbers, send the existing 
		# Port numbers back to the client depending on the communication type
		if(channel_name in channel_list):
			pub_port = channel_list[channel_name][0]
			sub_port = channel_list[channel_name][1]
			print ("Ports exist in dictionary:", pub_port, sub_port)

		else:
			# Get new port numbers/increment port numbers dynamically
			port = str(int(port) + 1)
			pub_port = port
			port = str(int(port) + 1)
			sub_port = port

			# Update dictionary/create new pair if it does not exist
			channel_list.update( { channel_name: (pub_port, sub_port, comm_type) } )

			# Start the forwarder process here for the channel
			newchannel = Process(target = forwader, args=(sub_port, pub_port))
			newchannel.start()

		# Call the Pack function to pack data and send reply back to client
		pack(comm_type, pub_port, sub_port, master_reply)
		print(channel_list)
################################################################################
# mat plot helper code from PySimpleGUI Github
################################################################################
def draw_figure(canvas, figure, loc=(0, 0)):
	figure_canvas_agg = FigureCanvasTkAgg(figure, canvas)
	figure_canvas_agg.draw()
	figure_canvas_agg.get_tk_widget().pack(side='top', fill='both', expand=1)
	return figure_canvas_agg

################################################################################
# Main Function with the GUI code
################################################################################
def main():
	global server_run, data, channel_list, master_port, channel_port_range
	global channel_port_base
	print("Tala v 3.0")
	
	master_port = data[0]
	channel_port_base = data[1]
	channel_port_range = data[2]

	sg.theme("Purple")

	# get figure from plot and extract height and width variables
	fig = plt.gcf()  # if using Pyplot then get the figure from the plot
	figure_x, figure_y, figure_w, figure_h = fig.bbox.bounds

	# Minimum functionality behind menu, currently only Exit has functionality
	menu_def = [['File', ['Open', 'Save', 'Exit']]]
	# Layout contains text, buttons, input, and canvas for graph
	layout = [[sg.Menu(menu_def)],
					 # Row 1
			[sg.Text("Master Port"), sg.InputText(master_port, key="masterPort", size=(10,5)),
			sg.Text("Channel Port Base"), sg.InputText(channel_port_base, key="channelBase", size=(10,5)),
			sg.Text("Channel Port Range"), sg.InputText(channel_port_range, key="channelRange", size=(10,5)),
			sg.Button('Load')],
			##################################################################
			# Node row
			# [sg.Text("Node Port Base"), sg.Input(key="nodeBase", size=(10,5)),
			# sg.Text("Node Port Range"), sg.Input(key="nodeRange", size=(10,5)),
			# sg.Button('Save')],
			##################################################################
			# row 2
			[sg.Text("Tala Server"), sg.Button("Stop"), sg.Button("Start")],
			# row 3
			[sg.Text("Active Nodes"), sg.Button('Show'), sg.Button('Kill Node'),
			sg.Text("Active Channels"), sg.Button('Show'), sg.Button('Kill Channel')],
			# row 4
			[sg.Input(size=(20,8)), sg.Text("           "), sg.Input(size=(20,8))],
			# row 5
			[sg.Canvas(size=(figure_w/2, figure_h/2), key='-CANVAS-')]]

	window = sg.Window("Tala", layout, force_toplevel=True, finalize=True)

	while True:
			event, values = window.read()
			if event in (None, "Exit"):
				server_run  = False
				break

			# global variables being set through read of button presses using index keys
			if event == "Load":
				# If user decides to provide their own values, clear default vals
				data.clear()
				master_port = values["masterPort"]
				channel_port_base = values["channelBase"]
				channel_port_range = values["channelRange"]

				# If master port is same as channel port 
				# Display a pop up error message
				if (master_port == channel_port_base):	
					sg.Popup("Master port cannot be the same with channel port base!")	
					# TODO: Clear/reset master port and channel port base	
				
				# Fill up data array with new values from user
				data.append(master_port)
				data.append(channel_port_base)
				data.append(channel_port_range)
				print("Data:", data)

			# if event == "Save":
			# 	node_port_base = values["nodeBase"]
			# 	node_port_range = values["nodeRange"]

			if event == "Start":
				print("Master Server Connecting...")
				server_run = True
				server_thr = thr.Thread(target=serverFunction, args=(1,))
				server_thr.start()

			# TODO: Handle the graph here
			# if event == "Show":
			# 	# Example dataframe to be graphed
			# 	df = pd.DataFrame({ 'from':['A', 'B', 'C','A'], 'to':['D', 'C', 'A','B']})

			# 	# Builds graph as G
			# 	G=nx.from_pandas_edgelist(df, 'from', 'to')

			# 	# Plot G, but does not show graph yet
			# 	# Node color can be changed through node_color using color name or hex value
			# 	# Node shape can be changed using node_shape can be one of 'so^>v<dph8' w/ o as default
			# 	# s=square, o=oval/circle, ^<v>=triangles, d=diamond, p=pentagon, h=hexagon, 8=octogon
			# 	nx.draw(G, node_color = 'green', node_shape = 'h', with_labels=True)
			# 	# draw graph on Canvas of GUI
			# 	fig_photo = draw_figure(window['-CANVAS-'].TKCanvas, fig)
			# # if event == "Show":
			# # if event == "Kill Node":
			# # if event == "Kill Channel":

			# Stop event
			if event == "Stop":
				print("Stopping server")
				server_run = False

	print("Cleaning up...")
	window.close()
	server_thr.join()
	print('Exit Tala')

server_run = False
# Intialize the data array with default values
data = [5000, 5050, 1000]
# Store the channels into a dictionary
channel_list = dict()
if __name__ == "__main__":
	main()