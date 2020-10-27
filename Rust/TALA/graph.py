import tkinter as tk
from tkinter import messagebox
from .TALA import Master, connect
import json
from .listbox import *


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
            for pubs in self.jsondata["channels"][key]["publishers"]:
                publishers.append(pubs)
            for subs in self.jsondata["channels"][key]["subscribers"]:
                subscribers.append(subs)
            self.channels[key] = [info, publishers,subscribers,cords]

    def connectMaster(self):
        self.master = connect(self.master_ip, self.master_port)

    def retrieveData(self):
        self.jsondata = json.loads(self.master.serialize())


class Graph(tk.Frame):

    def __init__(self, parent, controller, ip, port):
        tk.Frame.__init__(self,parent)
        self.connection = ConnectionData()
        self.connection.master_ip = ip
        self.connection.master_port = int(port)
        self.connection.connectMaster()
        self.parent = parent
        self.publishers = {}
        self.subscribers = {}
        self.channels = {}
        self.startGraph()

    def startGraph(self):
        self.canvas = ResizingCanvas(self,width=850, height=400, bg="#7a7f85", highlightthickness=0)
        self.canvas.place(relx = 0, rely = 0, relwidth = 1, relheight = 1,anchor = 'nw')
        self.channels.clear()

        self.connection.retrieveData()
        self.connection.parseJson()
        self.channels = self.connection.channels

        self.parseChannels()
        self.createGraph()
        self.leftButtons()
        self.rightButtons()



    def refresh(self):
        self.canvas.delete(all)
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
        self.canvas.create_line(start[0]+20, start[1]+10, end[0], end[1]+10, arrow=tk.LAST)

    def parseChannels(self):
        channel_count = 0
        x_channel = int(3*WIDTH / 6)
        y_channel = 0
        if len(self.channels) != 0:
            y_channel = int(HEIGHT / (len(self.channels)*2))

        for channel in self.channels:
            for pubs in self.channels[channel][1]:
                if pubs[1] not in self.publishers:
                    self.publishers[pubs[1]] = [pubs[0],[0,0]]
            for subs in self.channels[channel][2]:
                if subs[1] not in self.subscribers:
                    self.subscribers[subs[1]] = [subs[0],[0,0]]

            self.channels[channel][3] = [x_channel, y_channel * channel_count + 20]
            channel_count += 1

    def plotChannel(self):
        for name in self.channels:
            coords = self.channels[name][3]
            ip = "ip: " + str(self.channels[name][0][0])
            port = "port: " + str(self.channels[name][0][1])
            self.canvas.create_oval(coords[0], coords[1], coords[0]+20, \
            coords[1]+20, fill="#1ecbe1")
            self.canvas.create_text(coords[0] + 10, coords[1] - 10,  text=str(name), anchor='n')
            self.canvas.create_text(coords[0] + 10, coords[1] + 25,  text=str(ip), anchor='n')
            self.canvas.create_text(coords[0] + 10 , coords[1] + 35,  text=str(port), anchor='n')

    def plotPublishers(self):
        x_pub = int(1.5 * WIDTH / 6)
        y_pub = 0
        if len(self.publishers) != 0:
            y_pub = int(HEIGHT / (len(self.publishers)*2))
        object_count = 0
        for port in self.publishers:
            self.publishers[port][1] = [x_pub,y_pub*object_count+20]
            coords = self.publishers[port][1]
            ip = self.publishers[port][0]

            self.canvas.create_oval(coords[0], coords[1],coords[0]+20, coords[1]+20,\
             fill="#d926b6")
            self.canvas.create_text(coords[0]-10, coords[1],\
              text=str("port: " + str(port)), anchor='e')
            self.canvas.create_text(coords[0]-10, coords[1] + 10,\
              text=str("ip: " + ip), anchor='e')
            object_count += 1

    def plotSubscriber(self):
        x_sub = int(4.5 * WIDTH / 6)
        y_sub = 0
        if len(self.subscribers) != 0:
            y_sub = int(HEIGHT / (len(self.subscribers)*2))
        object_count = 0
        for port in self.subscribers:
            self.subscribers[port][1] = [x_sub,y_sub*object_count+20]
            coords = self.subscribers[port][1]
            ip = self.subscribers[port][0]

            self.canvas.create_oval(coords[0], coords[1],coords[0]+20, coords[1]+20,\
             fill="#d926b6")
            self.canvas.create_text(coords[0] + 30, coords[1],\
              text=str("port: " + str(port)), anchor='w')
            self.canvas.create_text(coords[0] + 30, coords[1] + 10,\
              text=str("ip: " + ip), anchor='w')
            object_count += 1



    def leftButtons(self):
        left_button_canvas = ResizingCanvas(self,width=850, height=400, bg="#7a7f85", highlightthickness=0)
        left_button_canvas.place(relx = 0, rely = 0, relwidth = .1, relheight = 1,anchor = 'nw')
        x_position = int(1*WIDTH / 6)

        # port_bot = tk.Button(left_button_canvas, text = "Port Ranges")
        # port_bot.place(x = 0, rely = .1, relwidth = 1, relheight = .05,anchor = 'w')

        list_sub_bot = tk.Button(left_button_canvas, text = "Terminate",command=lambda: self.terminate())
        list_sub_bot.place(x = 0, rely = .1, relwidth = 1, relheight = .05,anchor = 'w')

        list_pub_bot = tk.Button(left_button_canvas, text = "List Publishers",command=lambda: self.listPublishers())
        list_pub_bot.place(x = 0, rely = .3, relwidth = 1, relheight = .05,anchor = 'w')

        list_chan_bot = tk.Button(left_button_canvas, text = "List Channels",command=lambda: self.listChannel())
        list_chan_bot.place(x = 0, rely = .5, relwidth = 1, relheight = .05,anchor = 'w')

        list_sub_bot = tk.Button(left_button_canvas, text = "List Subscribers",command=lambda: self.listSubscribers())
        list_sub_bot.place(x = 0, rely = .7, relwidth = 1, relheight = .05,anchor = 'w')




    def terminate(self):
        self.connection.master.terminate()
        exit()



    def rightButtons(self):
        remove = tk.StringVar()
        right_button_canvas = ResizingCanvas(self,width=850, height=400, bg="#7a7f85", highlightthickness=0)
        right_button_canvas.place(relx = 1, rely = 1, relwidth = .1, relheight = 1,anchor = 'se')
        x_position = int(WIDTH)

        remove_entry = tk.Entry(self, bg = 'white', textvariable = remove)
        remove_entry.place(relx = 1, rely = .1, relwidth = .1, relheight = .05,anchor = 'e')

        delete_chan_bot = tk.Button(right_button_canvas, text = "Remove Channel", command=lambda: self.removeChan(remove))
        delete_chan_bot.place(relx = 1, rely = .7, relwidth = 1, relheight = .05,anchor = 'e',)

        refresh_bot = tk.Button(right_button_canvas, text = "Refresh", command=lambda: self.refresh())
        refresh_bot.place(relx = .9, rely = .9, relwidth = 1, relheight = .05,anchor = 'e',)


    def listChannel(self):
        list = MultiListbox(self, ['Name','IP', 'Port'], width = 10,highlightthickness=0, border=0)
        data = []

        for key in self.channels:
            data.append(key)
            data.append(self.channels[key][0][0])
            data.append(self.channels[key][0][1])


        list.add_data(data)
        list.place(relx = 0, rely = 1, anchor = 'sw')

    def listPublishers(self):
        list = MultiListbox(self, ['IP', 'Port'], width = 15,highlightthickness=0, border=0)
        data = []

        for key in self.publishers:
            data.append(self.publishers[key][0])
            data.append(key)


        list.add_data(data)
        list.place(relx = 0, rely = 1, anchor = 'sw')

    def listSubscribers(self):
        list = MultiListbox(self, ['IP', 'Port'], width = 15,highlightthickness=0, border=0)
        data = []

        for key in self.subscribers:
            data.append(self.subscribers[key][0])
            data.append(key)

        list.add_data(data)
        list.place(relx = 0, rely = 1, anchor = 'sw')


    def removeChan(self,remove):
        channel = str(remove.get())
        try:
            self.connection.master.removeChannel(channel)
            self.refresh()
        except:
            tk.messagebox.showerror("Error", "Invalid Channel")
# Resizingcanvas
# A TK Canvas class that resizes a canvas and
# its elements when a user resizes a window.
class ResizingCanvas(tk.Canvas):
    def __init__(self,parent,**kwargs):

        tk.Canvas.__init__(self,parent,**kwargs)
        self.bind("<Configure>", self.on_resize)
        self.height = self.winfo_reqheight()
        self.width = self.winfo_reqwidth()

    def on_resize(self,event):
        # determine the ratio of old width/height to new width/height
        wscale = float(event.width)/self.width
        hscale = float(event.height)/self.height
        self.width = event.width
        self.height = event.height
        # resize the canvas
        self.config(width=self.width, height=self.height)
        # rescale all the objects tagged with the "all" tag
        self.scale("all",0,0,wscale,hscale)
