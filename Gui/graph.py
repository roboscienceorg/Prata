import numpy as np
import tkinter as tk

HEIGHT = 700
WIDTH = 800
root = tk.Tk()

pub1 = np.array(["192.168.1.1","192.168.1.3","192.168.1.2"])
pub2 = np.array(["192.168.1.3","192.168.1.4","192.168.1.5","192.168.2.3","192.168.2.4","192.168.2.5"])
sub1 = np.array(["192.168.2.1","192.168.2.2","192.168.2.3"])
cords = [400,90]
channel1 = np.array([pub1,sub1,cords])
channel2 = np.array([pub2,sub1,cords])
channels = {"channel1":channel1,"channel2":channel2}

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

class graphing(tk.Canvas):
    def __init__(self,parent,**kwargs):
        tk.Canvas.__init__(self,parent,**kwargs)


    def calculatePoint(self,canvas,object_dic,x_offset,color):
        x_pub = int(WIDTH / x_offset)
        y_pub = int(HEIGHT / (len(object_dic)*2))
        object_count = 0
        for ip in object_dic:
            object_dic[ip] = [x_pub,y_pub*object_count+20]
            self.plotPoint(canvas,object_dic[ip],ip,color)
            object_count += 1

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


    def drawArrow(self,canvas,start,end):
        canvas.create_line(start[0]+20, start[1]+10, end[0], end[1]+10, arrow=tk.LAST)

   

    def plotPoint(self,canvas,dic,type,color):
            canvas.create_oval(dic[0], dic[1], dic[0]+20, \
                dic[1]+20, fill=color)
            canvas.create_text(dic[0]-15, dic[1],  text=str(type), anchor='e')


def myframe():
	myframe = tk.Frame(root)
	myframe.pack(fill="both", expand=True)
	canvas = ResizingCanvas(myframe,width=850, height=400, bg="white", highlightthickness=0)
	canvas.pack(fill="both", expand=True)

	graph = graphing(canvas)
	publihsers = graph.createGraph(channels,canvas)

	root.mainloop()




myframe()