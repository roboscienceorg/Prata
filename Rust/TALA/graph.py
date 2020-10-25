import tkinter as tk
from tkinter import messagebox
from PIL import Image, ImageTk
from .TALA import Master, connect
import json
from .listbox import *
from .buttons import *
from .resizingcanvas import *


HEIGHT = 700
WIDTH = 800
MASTERIP = ""
MASTERPORT = 0



class ConnectionData():
    def __init__(self):
        self.channels = {}
        self.jsondata = {}
        self.master_ip = ""
        self.master_port = 0

    def parseJson(self):
        cords = [0,0]
        publishers = []
        subscribers = []
        for key in self.jsondata["channels"]:
            publishers = []
            subscribers = []
            info = self.jsondata["channels"][key]["info"]
            publishers = self.jsondata["channels"][key]["publishers"]
            subscribers = self.jsondata["channels"][key]["subscribers"]
            self.channels[key] = [info, publishers,subscribers,cords]

    def connectMaster(self):
        self.master = connect(self.master_ip, self.master_port)

    def retrieveData(self):
        self.jsondata = json.loads(self.master.serialize())


class Graph(tk.Frame):

    def __init__(self, parent, controller, ip, port):
        tk.Frame.__init__(self,parent)
        self.buttons = Buttons(self)
        self.connection = ConnectionData()
        self.connection.master_ip = ip
        self.connection.master_port = int(port)
        self.connection.connectMaster()
        self.parent = parent
        self.controller = controller
        self.publishers = {}
        self.subscribers = {}
        self.channels = {}
        self.startGraph()



    #   createGraph(self,channels,canvas)
    #       Takes in a dictionary of channels and a canvas.It then
    #       Seperates out the publishers and subscribers and creates a
    #       new dictionary for each that contains all the channels they are
    #       connected to and an array for the x and y position on the graph.
    #       It then calls calculatrePoint passing in the publisher and subsciber
    #       Dictionaries. After, it then draws arrows from the publishers to channels
    #       and the Channels to the subscribers
    def createGraph(self):
        self.loadimage = tk.PhotoImage(file="button.png")
        self.loadimage = self.loadimage.subsample(5,5)
        self.plotChannel()
        self.plotPublishers()
        self.plotSubscriber()

        for channel in self.channels:
            for pubs in self.channels[channel][1]:
                self.drawArrow(self.publishers[pubs[1]][1],self.channels[channel][3])
            for subs in self.channels[channel][2]:
                self.drawArrow(self.channels[channel][3],self.subscribers[subs[1]][1])


    #   drawArror(self,canvas,start,end)
    #   Takes in a canvas and the start and end points of the arror.
    #   Then draws an arrow from the starting point to the ending point.
    def drawArrow(self,start,end):
        self.canvas.create_line(start[0] + 10, start[1]+7, end[0] - 10, end[1]+7, arrow=tk.LAST)


    def displayMasterInfo(self):
        FONT= ("Verdana", 20)

        master_ip = "Master IP: " + str(self.connection.master_ip)
        master_port = "Master Port: " + str(self.connection.master_port)

        IP_label = tk.Label(self, text = master_ip, bg = "#7a7f85", font = FONT)
        IP_label.place(relx = .5, rely = .05, relwidth = .25, relheight = .1 ,anchor = 'e')

        Port_label = tk.Label(self, text = master_port, bg = "#7a7f85", font = FONT)
        Port_label.place(relx = .5, rely = .05, relwidth = .25, relheight = .1 ,anchor = 'w')

    def parseChannels(self):
        channel_count = 0
        x_channel = int(self.canvas.width / 2)
        y_channel = 0
        if len(self.channels) != 0:
            y_channel = int((self.canvas.height*.9)  / (len(self.channels)))

        for channel in self.channels:
            for pubs in self.channels[channel][1]:
                if pubs[1] not in self.publishers:
                    self.publishers[pubs[1]] = [pubs[0],[0,0]]
            for subs in self.channels[channel][2]:
                if subs[1] not in self.subscribers:
                    self.subscribers[subs[1]] = [subs[0],[0,0]]

            self.channels[channel][3] = [x_channel, y_channel * channel_count + 50]
            channel_count += 1


    def plotChannel(self):

        channel_count = 0

        for name in self.channels:
            coords = self.channels[name][3]

            rel_coords = [0,0]
            rel_coords[0] = .5
            rel_coords[1] = (.9 / len(self.channels)) * channel_count + 50/self.canvas.height

            ip = "ip: " + str(self.channels[name][0][0])
            port = "port: " + str(self.channels[name][0][1])
            print(name)
            channel_bot = tk.Button(self.canvas, bg="#1ecbe1", command=lambda i = name: self.buttons.displayChannel(i))
            channel_bot.place(relx = rel_coords[0],rely = rel_coords[1], width = 30, height = 30,anchor = 'n')

            self.canvas.create_text(coords[0] , coords[1] - 10,  text=str(name), anchor='n')
            self.canvas.create_text(coords[0] , coords[1] + 25,  text=str(ip), anchor='n')
            self.canvas.create_text(coords[0] , coords[1] + 35,  text=str(port), anchor='n')
            channel_count += 1


    def plotPublishers(self): 

        x_pub = int(1.5 * self.canvas.width / 6)
        y_pub = 0
        object_count = 0


        if len(self.publishers) != 0:
            y_pub = int((self.canvas.height*.9)  / (len(self.publishers)))

        for port in self.publishers:
            self.publishers[port][1] = [x_pub,y_pub*object_count+50]
            coords = self.publishers[port][1]

            rel_coords = [0,0]
            rel_coords[0] = .25
            rel_coords[1] = (.9 / len(self.publishers)) * object_count + 50/self.canvas.height

            ip = self.publishers[port][0]

            # self.canvas.create_oval(coords[0]-10, coords[1],coords[0]+10, coords[1]+20,\
            #  fill="#d926b6")
  
            self.roundedbutton = tk.Button(self.canvas, image=self.loadimage, bg = "#7a7f85",borderwidth = 0,\
                command=lambda i = port: self.buttons.displayPublishers(i))
            self.roundedbutton.place(relx = rel_coords[0],rely = rel_coords[1], width = 30, height = 30,anchor = 'n')

            self.canvas.create_text(coords[0] - 15, coords[1], text=str("port: " + str(port)), anchor='e')
            self.canvas.create_text(coords[0] - 15, coords[1] + 10, text=str("ip: " + ip), anchor='e')
            object_count += 1


    def plotSubscriber(self):

        x_sub = int(4.5 * self.canvas.width / 6)
        y_sub = 0
        object_count = 0

        if len(self.subscribers) != 0:
            y_sub = int(HEIGHT / (len(self.subscribers)*2))
        for port in self.subscribers:
            self.subscribers[port][1] = [x_sub,y_sub*object_count+50]
            coords = self.subscribers[port][1]
            ip = self.subscribers[port][0]

            rel_coords = [0,0]
            rel_coords[0] = .75
            rel_coords[1] = (.9 / len(self.publishers)) * object_count + 50/self.canvas.height

            self.roundedbutton = tk.Button(self.canvas, image=self.loadimage, bg = "#7a7f85",borderwidth = 0,\
                command=lambda i = port: self.buttons.displaySubscribers(i))
            self.roundedbutton.place(relx = rel_coords[0],rely = rel_coords[1], width = 30, height = 30,anchor = 'n')

            self.canvas.create_text(coords[0] + 15, coords[1], text=str("port: " + str(port)), anchor='w')
            self.canvas.create_text(coords[0] + 15, coords[1] + 10, text=str("ip: " + ip), anchor='w')
            object_count += 1


    def channelshow(self,port):
        print(self.subscribers[port])

    def refresh(self):
        self.canvas.delete(all)
        self.controller.createDisplay()


    def startGraph(self):

        self.canvas = ResizingCanvas(self,width=850, height=400, bg="#7a7f85", highlightthickness=0)
        self.canvas.place(x = 0, y = 0, relwidth = 1, relheight = 1,anchor = 'nw')

        self.displayMasterInfo()
        self.channels.clear()

        self.connection.retrieveData()
        self.connection.parseJson()
        self.channels = self.connection.channels

        self.parseChannels()
        self.createGraph()
        self.buttons.leftButtons()
        self.buttons.rightButtons()





    

