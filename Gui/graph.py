import numpy as np
import tkinter as tk

HEIGHT = 700
WIDTH = 800
root = tk.Tk()

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

def seperateData():
	pub1 = np.array(["192.168.1.1","192.168.1.2","192.168.1.3"])
	pub2 = np.array(["192.168.1.3","192.168.1.4","192.168.1.5"])
	sub1 = np.array(["192.168.2.1","192.168.2.2","192.168.2.2"])
	pubs = np.array([pub1,pub2])
	subs = np.array([sub1])
	publishers = np.array([])
	channels = np.array(["192.168.3.1"])
	subscribers = np.array([])

	for x in range(0,len(pubs)):
		for y in pubs[x]:
			if y not in publishers:
				publishers = np.append(publishers, y)

	for x in range(0,len(subs)):
		for y in subs[x]:
			if y not in subscribers:
				subscribers = np.append(subscribers, y)

	return publishers, channels, subscribers

def plot(publishers,canvas,x_offset,y_offset,type):
	y = int(HEIGHT / (len(publishers)*2))
	x = int(WIDTH / 6)
	for i in range(len(publishers)):
		canvas.create_oval(x+x_offset, y*i+y_offset, x+x_offset+20, \
			y*i+y_offset+20, fill="black")
		canvas.create_text(x+x_offset, y*i+y_offset-15,  text=str(type)+str(i), anchor='n')




myframe = tk.Frame(root)
myframe.pack(fill="both", expand=True)
canvas = ResizingCanvas(myframe,width=850, height=400, bg="white", highlightthickness=0)
canvas.pack(fill="both", expand=True)

publishers,channels,subscribers = seperateData()	

plot(publishers,canvas,50,50,"Pub")
plot(channels,canvas,325,50,"channel")
plot(subscribers,canvas,650,50,"Sub")

root.mainloop()