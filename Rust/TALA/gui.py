import tkinter as tk
from PIL import Image, ImageTk
from .graph import *
from os.path import split
loc = split(__file__)[0]

ansuzPNG = loc + "\\ansuz.png"
ansuzICO = loc + "\\ansuz.ico"


LARGE_FONT= ("Verdana", 20)


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

        self.ip = tk.StringVar()
        self.port = tk.StringVar()
        self.parent = parent
        self.controller = controller
        self.createDisplay()


    def createDisplay(self,):
        img = ImageTk.PhotoImage(file=ansuzPNG)

        canvas = tk.Canvas(self, width = img.width(), height = img.height())
        canvas.place(relx = .5, rely = .45, relwidth = 1, relheight = 1,anchor = 'center')

        self.parent.one = img
        canvas.create_image(0,0, anchor='nw', image=img)
        canvas.create_text(img.width()/2,100,fill="black",font=LARGE_FONT,text="TALA")

        create_label = tk.Label(self, text = "Create new Host", bg = "white")
        create_label.place(x = img.width()/2, rely = .45, relwidth = .1, relheight = .05 ,anchor = 'n')

        create_bot = tk.Button(self, text = "Create")
        create_bot.place(x = img.width()/2, rely = .5, relwidth = .1, relheight = .05,anchor = 'n')

        ip_label = tk.Label(self, text = "IP of Host", bg = "white")
        ip_label.place(x = img.width()/2, rely = .65, width = 100, height = 25 ,anchor = 's')

        ip_entry = tk.Entry(self, bg = 'white', textvariable = self.ip)
        ip_entry.place(x = img.width()/2, rely = .65, relwidth = .1, relheight = .05,anchor = 'n')

        port_label = tk.Label(self, text = "Port of Host", bg = "white")
        port_label.place(x = img.width()/2, rely = .75, width = 100, height = 25 ,anchor = 's')

        port_entry = tk.Entry(self, bg = 'white', textvariable = self.port)
        port_entry.place(x = img.width()/2, rely = .75, relwidth = .1, relheight = .05,anchor = 'n')

        connect_bot = tk.Button(self, text = "Connect", command=lambda: [self.setMaster()])
        connect_bot.place(x = img.width()/2, rely = .85, relwidth = .1, relheight = .05,anchor = 'n')


    def setMaster(self):
        self.master_ip = self.ip.get()
        self.master_port = self.port.get()

        try:
            frame = Graph(self.parent, self, self.master_ip, self.master_port)
            self.controller.frames[Graph] = frame
            frame.grid(row=0, column=0, sticky="nsew")
            self.controller.topFrame(Graph)
        except:
            print("Error")
            tk.messagebox.showerror("Error", "The combination IP and port are invalid. \nPlease Re-enter and try again")
            self.createDisplay()

def gui():
    app = ManageFrames()
    app.mainloop()
