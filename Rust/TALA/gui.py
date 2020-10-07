import tkinter as tk
from PIL import Image, ImageTk
from graph import *


LARGE_FONT= ("Verdana", 20)


class ManageFrames(tk.Tk):

    def __init__(self, *args, **kwargs):
        tk.Tk.__init__(self, *args, **kwargs)
        tk.Tk.wm_title(self, "Tala")
        tk.Tk.iconbitmap(self, default = "ansuz.ico")
        self.geometry("1300x800")
        frame_container = tk.Frame(self)
        self.frames = {}


        frame_container.grid_rowconfigure(0, weight=1)
        frame_container.grid_columnconfigure(0, weight=1)
        frame_container.place(relx = .5, rely = 0.5, relwidth = 1, relheight = 1,anchor = 'center')


        for page in (Window,Graph):

            frame = page(frame_container, self)

            self.frames[page] = frame

            frame.grid(row=0, column=0, sticky="nsew")

        self.topFrame(Window)

    def topFrame(self, cont):

        frame = self.frames[cont]
        frame.tkraise()






class Window(tk.Frame):
    def __init__(self, parent, controller):
        tk.Frame.__init__(self,parent)

        img = ImageTk.PhotoImage(file="ansuz.png")

        canvas = tk.Canvas(self, width = img.width(), height = img.height())
        canvas.place(relx = .5, rely = .45, relwidth = 1, relheight = 1,anchor = 'center')

        parent.one = img
        canvas.create_image(0,0, anchor='nw', image=img)

        canvas.create_text(img.width()/2,100,fill="black",font=LARGE_FONT,
                        text="TALA")


        create_label = tk.Label(self, text = "Create new Host", bg = "white")
        create_label.place(x = img.width()/2, rely = .45, relwidth = .1, relheight = .05 ,anchor = 'n')

        create_bot = tk.Button(self, text = "Create")
        create_bot.place(x = img.width()/2, rely = .5, relwidth = .1, relheight = .05,anchor = 'n')

        canvas.create_text(img.width()/2,100,fill="black",font=LARGE_FONT,
                        text="TALA")

        ip_label = tk.Label(self, text = "IP of Host", bg = "white")
        ip_label.place(x = img.width()/2, rely = .65, width = 100, height = 25 ,anchor = 's')

        ip_entry = tk.Entry(self, bg = 'white')
        ip_entry.place(x = img.width()/2, rely = .65, relwidth = .1, relheight = .05,anchor = 'n')

        port_label = tk.Label(self, text = "Port of Host", bg = "white")
        port_label.place(x = img.width()/2, rely = .75, width = 100, height = 25 ,anchor = 's')

        port_entry = tk.Entry(self, bg = 'white')
        port_entry.place(x = img.width()/2, rely = .75, relwidth = .1, relheight = .05,anchor = 'n')

        connect_bot = tk.Button(self, text = "Connect", command=lambda: controller.topFrame(Graph))
        connect_bot.place(x = img.width()/2, rely = .85, relwidth = .1, relheight = .05,anchor = 'n')



def gui(json_object):
    #parse json DATA
    # needs master.rs data struct
    print("Made it to GUI here is the current Master DATA:")
    print(json_object)
app = ManageFrames()
app.mainloop()
