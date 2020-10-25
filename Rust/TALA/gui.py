import tkinter as tk
from .graph import *
from .resizingcanvas import *
from os.path import split
loc = split(__file__)[0]

ansuzPNG = loc + "\\ansuz.png"
ansuzICO = loc + "\\ansuz.ico"


LARGE_FONT= ("Verdana", 100)


class ManageFrames(tk.Tk):

    def __init__(self, *args, **kwargs):
        tk.Tk.__init__(self, *args, **kwargs)
        tk.Tk.wm_title(self, "Tala")
        tk.Tk.iconbitmap(self, default = ansuzICO)
        self.geometry("1300x800")
        frame_container = tk.Frame(self)
        self.frames = {}


        frame_container.grid_rowconfigure(0, weight=1)
        frame_container.grid_columnconfigure(0, weight=1)
        frame_container.place(relx = .5, rely = 0.5, relwidth = 1, relheight = 1,anchor = 'center')

        frame = Window(frame_container, self)

        self.frames[Window] = frame

        frame.grid(row=0, column=0, sticky="nsew")

        self.topFrame(Window)

    def topFrame(self, cont):

        frame = self.frames[cont]
        frame.tkraise()






class Window(tk.Frame):
    def __init__(self, parent, controller):
        tk.Frame.__init__(self,parent)
        self.canvas = ResizingCanvas(self,width=850, height=400, bg="#1ecbe1", highlightthickness=0)
        self.canvas.place(relx = 0, rely = 0, relwidth = 1, relheight = 1,anchor = 'nw')
        self.ip = tk.StringVar()
        self.port = tk.StringVar()
        self.parent = parent
        self.controller = controller
        self.createDisplay()


    def createDisplay(self):


        create_label = tk.Label(self.canvas, text = "TALA", bg = "#1ecbe1",font = LARGE_FONT)
        create_label.place(relx = .5, rely = .2, relwidth = .5, relheight = .2 ,anchor = 'center')

        create_label = tk.Label(self.canvas, text = "Create new Host", bg = "white")
        create_label.place(relx = .5, rely = .45, relwidth = .1, relheight = .05 ,anchor = 'n')

        create_bot = tk.Button(self.canvas, text = "Create")
        create_bot.place(relx = .5, rely = .5, relwidth = .1, relheight = .05,anchor = 'n')

        ip_label = tk.Label(self.canvas, text = "IP of Host", bg = "white")
        ip_label.place(relx = .5, rely = .65, relwidth = .1,  relheight = .05 ,anchor = 's')

        ip_entry = tk.Entry(self.canvas, bg = 'white', textvariable = self.ip)
        ip_entry.place(relx = .5, rely = .65, relwidth = .1, relheight = .05,anchor = 'n')

        port_label = tk.Label(self.canvas, text = "Port of Host", bg = "white")
        port_label.place(relx = .5, rely = .75, relwidth = .1,  relheight = .05 ,anchor = 's')

        port_entry = tk.Entry(self.canvas, bg = 'white', textvariable = self.port)
        port_entry.place(relx = .5, rely = .75, relwidth = .1, relheight = .05,anchor = 'n')

        connect_bot = tk.Button(self.canvas, text = "Connect", command=lambda: [self.setMaster()])
        connect_bot.place(relx = .5, rely = .85, relwidth = .1, relheight = .05,anchor = 'n')


    def setMaster(self):
        self.master_ip = self.ip.get()
        self.master_port = self.port.get()

        self.master_ip = "127.0.0.1"
        self.master_port = 25565


        frame = Graph(self.parent, self, self.master_ip, self.master_port)
        self.controller.frames[Graph] = frame
        frame.grid(row=0, column=0, sticky="nsew")
        self.controller.topFrame(Graph)
        # except:
        #     print("Error")
        #     tk.messagebox.showerror("Error", "The combination IP and port are invalid. \nPlease Re-enter and try again")
        #     self.createDisplay()

def gui():
    app = ManageFrames()
    app.mainloop()
