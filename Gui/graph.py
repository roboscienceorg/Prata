import numpy as np
import tkinter as tk

HEIGHT = 700
WIDTH = 800
root = tk.Toplevel()

pub1 = np.array(["192.168.1.1","192.168.1.3","192.168.1.2"])
pub2 = np.array(["192.168.1.3","192.168.1.4","192.168.1.5","192.168.2.3","192.168.2.4","192.168.2.5"])
sub1 = np.array(["192.168.2.1","192.168.2.2","192.168.2.3"])
cords = [400,90]
channel1 = np.array([pub1,sub1,cords])
channel2 = np.array([pub2,sub1,cords])
channels = {"channel1":channel1,"channel2":channel2}


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


# Graphing
# A class that will take the current channel,
# publishers, and subscriber and graph them into 
# a new window. 
#
# Function:
#   calculatePoints(self,canvas,object_dic,x_offset,color)
#     Takes in a canvas, a dictionary, the x offset and the color
#     of the nodes in the graph. With this information it calculates
#     the spacing of the graph and calls plotPoint to plot the points to 
#     a canvas.

#   createGraph(self,channels,canvas)
#       Takes in a dictionary of channels and a canvas.It then 
#       Seperates out the publishers and subscribers and creates a
#       new dictionary for each that contains all the channels they are
#       connected to and an array for the x and y position on the graph.
#       It then calls calculatrePoint passing in the publisher and subsciber
#       Dictionaries. After, it then draws arrows from the publishers to channels
#       and the Channels to the subscribers 

#   drawArror(self,canvas,start,end)
#   Takes in a canvas and the start and end points of the arror.
#   Then draws an arrow from the starting point to the ending point.

#   plotPoint(self,canvas,dic,text,color)
#   Takes in a canvas, a dictionary, the text, and the color of the node
#   It then draws ovals of diameter 20 on the x and y coordinates of the 
#   dictionary.
class Graphing(tk.Canvas):
    def __init__(self,parent,**kwargs):
        tk.Canvas.__init__(self,parent,**kwargs)

    #   calculatePoints(self,canvas,object_dic,x_offset,color)
    #     Takes in a canvas, a dictionary, the x offset and the color
    #     of the nodes in the graph. With this information it calculates
    #     the spacing of the graph and calls plotPoint to plot the points to 
    #     a canvas.
    def calculatePoint(self,canvas,object_dic,x_offset,color):
        x_pub = int(WIDTH / x_offset)
        y_pub = int(HEIGHT / (len(object_dic)*2))
        object_count = 0
        for ip in object_dic:
            object_dic[ip] = [x_pub,y_pub*object_count+20]
            self.plotPoint(canvas,object_dic[ip],ip,color)
            object_count += 1
#   createGraph(self,channels,canvas)
#       Takes in a dictionary of channels and a canvas.It then 
#       Seperates out the publishers and subscribers and creates a
#       new dictionary for each that contains all the channels they are
#       connected to and an array for the x and y position on the graph.
#       It then calls calculatrePoint passing in the publisher and subsciber
#       Dictionaries. After, it then draws arrows from the publishers to channels
#       and the Channels to the subscribers

    def createGraph(self,channels,canvas):
        publishers = {}
        subscribers = {}
        channel_count = 0
        x_channel = int(WIDTH / 2)
        y_channel = int(HEIGHT / (len(channels)*2))

        for channel in channels:
            for pubs in channels[channel][0]:
                if pubs not in publishers:
                    publishers[pubs] = [0,0]

            for subs in channels[channel][1]:
                if subs not in subscribers:
                    subscribers[subs] = [0,0]

            channels[channel][2] = [x_channel, y_channel * channel_count + 20]
            self.plotPoint(canvas,channels[channel][2],channel,"#1ecbe1")
            channel_count += 1

    
        self.calculatePoint(canvas,publishers,6,"#d926b6")
        self.calculatePoint(canvas,subscribers,1,"#26D949")

        for channel in channels:
            for pubs in channels[channel][0]:
                self.drawArrow(canvas,publishers[pubs],channels[channel][2])
            for subs in channels[channel][1]:
                self.drawArrow(canvas,channels[channel][2],subscribers[subs])

#   drawArror(self,canvas,start,end)
#   Takes in a canvas and the start and end points of the arror.
#   Then draws an arrow from the starting point to the ending point.
    def drawArrow(self,canvas,start,end):
        canvas.create_line(start[0]+20, start[1]+10, end[0], end[1]+10, arrow=tk.LAST)

   
#   plotPoint(self,canvas,dic,text,color)
#   Takes in a canvas, a dictionary, the text, and the color of the node
#   It then draws ovals of diameter 20 on the x and y coordinates of the 
#   dictionary.
    def plotPoint(self,canvas,dic,text,color):
            canvas.create_oval(dic[0], dic[1], dic[0]+20, \
                dic[1]+20, fill=color)
            canvas.create_text(dic[0]-15, dic[1],  text=str(text), anchor='e')

def show_frame(self, page_name):
    '''Show a frame for the given page name'''
    frame = self.frames[page_name]
    frame.tkraise()

def myframe():
	myframe = tk.Frame(root)
	myframe.pack(fill="both", expand=True)
	canvas = ResizingCanvas(myframe,width=850, height=400, bg="white", highlightthickness=0)
	canvas.pack(fill="both", expand=True)
	graph = Graphing(canvas)
    graph.createGraph(channels,canvas)
	root.mainloop()




# myframe()