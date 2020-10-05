import numpy as np
import tkinter as tk

HEIGHT = 700
WIDTH = 800

pub1 = np.array(["192.168.1.1","192.168.1.3","192.168.1.2"])
pub2 = np.array(["192.168.1.3","192.168.1.4","192.168.1.5","192.168.2.3","192.168.2.4","192.168.2.5","192.168.3.3","192.168.3.4","192.168.3.5"])
sub1 = np.array(["192.168.2.1","192.168.2.2","192.168.2.3"])
cords = [400,90]
channel1 = np.array([pub1,sub1,cords], dtype=object)
channel2 = np.array([pub2,sub1,cords], dtype=object)
channels = {"channel1":channel1,"channel2":channel2}


class Graph(tk.Frame):

    def __init__(self, parent, controller):
        tk.Frame.__init__(self,parent)
        self.canvas = ResizingCanvas(self,width=850, height=400, bg="#7a7f85", highlightthickness=0)
        self.canvas.place(relx = 0, rely = 0, relwidth = 1, relheight = 1,anchor = 'nw')
        self.publishers = {}
        self.subscribers = {}
        self.createGraph(channels)
        self.buttons()

    def calculatePoint(self,object_dic,x_offset,color):
        x_pub = int(x_offset*WIDTH / 6)
        y_pub = int(HEIGHT / (len(object_dic)*2))
        object_count = 0
        for ip in object_dic:
            object_dic[ip] = [x_pub,y_pub*object_count+20]
            self.plotPoint(object_dic[ip],ip,color)
            object_count += 1

    #   createGraph(self,channels,canvas)
    #       Takes in a dictionary of channels and a canvas.It then 
    #       Seperates out the publishers and subscribers and creates a
    #       new dictionary for each that contains all the channels they are
    #       connected to and an array for the x and y position on the graph.
    #       It then calls calculatrePoint passing in the publisher and subsciber
    #       Dictionaries. After, it then draws arrows from the publishers to channels
    #       and the Channels to the subscribers

    def createGraph(self,channels):

        channel_count = 0
        x_channel = int(3*WIDTH / 6)
        y_channel = int(HEIGHT / (len(channels)*2))
        for channel in channels:
            for pubs in channels[channel][0]:
                if pubs not in self.publishers:
                    self.publishers[pubs] = [0,0]

            for subs in channels[channel][1]:
                if subs not in self.subscribers:
                    self.subscribers[subs] = [0,0]

            channels[channel][2] = [x_channel, y_channel * channel_count + 20]
            self.plotPoint(channels[channel][2],channel,"channel")
            channel_count += 1

    
        self.calculatePoint(self.publishers,1.5,"publisher")
        self.calculatePoint(self.subscribers,4.5,"subsciber")

        for channel in channels:
            for pubs in channels[channel][0]:
                self.drawArrow(self.publishers[pubs],channels[channel][2])
            for subs in channels[channel][1]:
                self.drawArrow(channels[channel][2],self.subscribers[subs])


    #   drawArror(self,canvas,start,end)
    #   Takes in a canvas and the start and end points of the arror.
    #   Then draws an arrow from the starting point to the ending point.
    def drawArrow(self,start,end):
        self.canvas.create_line(start[0]+20, start[1]+10, end[0], end[1]+10, arrow=tk.LAST)

   
    #   plotPoint(self,canvas,dic,text,color)
    #   Takes in a canvas, a dictionary, the text, and the color of the node
    #   It then draws ovals of diameter 20 on the x and y coordinates of the 
    #   dictionary.
    def plotPoint(self,dic,text,type):
        if(type == "channel"):
            self.canvas.create_oval(dic[0], dic[1], dic[0]+20, \
            dic[1]+20, fill="#1ecbe1")
            self.canvas.create_text(dic[0], dic[1]-10,  text=str(text), anchor='n')
        elif(type == "publisher"):
            self.canvas.create_oval(dic[0], dic[1], dic[0]+20, \
            dic[1]+20, fill="#d926b6")
            self.canvas.create_text(dic[0]-10, dic[1],  text=str(text), anchor='e')
        elif(type == "subsciber"):
            self.canvas.create_oval(dic[0], dic[1], dic[0]+20, \
            dic[1]+20, fill="#d926b6")
            self.canvas.create_text(dic[0]+15, dic[1],  text=str(text), anchor='w')

    def buttons(self):
        x_position = int(1*WIDTH / 6)

        create_bot = tk.Button(self, text = "Port Ranges")
        create_bot.place(x = 0, rely = .1, relwidth = .1, relheight = .05,anchor = 'w')

        create_bot = tk.Button(self, text = "List Publishers",command=lambda: self.listPublishers())
        create_bot.place(x = 0, rely = .3, relwidth = .1, relheight = .05,anchor = 'w')
      
        create_bot = tk.Button(self, text = "List Channels",command=lambda: self.listChannel())
        create_bot.place(x = 0, rely = .5, relwidth = .1, relheight = .05,anchor = 'w')

        create_bot = tk.Button(self, text = "List Subscribers",command=lambda: self.listSubscribers())
        create_bot.place(x = 0, rely = .7, relwidth = .1, relheight = .05,anchor = 'w')

        create_bot = tk.Button(self, text = "Show BlackList")
        create_bot.place(x = 0, rely = .9, relwidth = .1, relheight = .05,anchor = 'w',)

    def listPublishers(self):
        x_position = int(WIDTH)

        list = tk.Listbox(self.canvas)
        for key in self.publishers:
            print(key)
            list.insert('end',key)
        list.place(relx = 1, y = 10, anchor = 'ne')
 
    def listChannel(self):
        x_position = int(WIDTH)

        list = tk.Listbox(self.canvas)
        for key in channels:
            print(key)
            list.insert('end',key)
        list.place(relx = 1, y = 10, anchor = 'ne')

    def listSubscribers(self):
        x_position = int(WIDTH)

        list = tk.Listbox(self.canvas)
        for key in self.subscribers:
            print(key)
            list.insert('end',key)
        list.place(relx = 1, y = 10, anchor = 'ne')

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