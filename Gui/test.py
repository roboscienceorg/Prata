import tkinter as tk
from PIL import Image, ImageTk


LARGE_FONT= ("Verdana", 12)


class ManageFrames(tk.Tk):

    def __init__(self, *args, **kwargs):
        
        tk.Tk.__init__(self, *args, **kwargs)
        frame_container = tk.Frame(self)
        self.frames = {}


        frame_container.grid_rowconfigure(0, weight=1)
        frame_container.grid_columnconfigure(0, weight=1)
        frame_container.place(relx = .5, rely = 0.05, relwidth = 1, relheight = 1,anchor = 'n')


        for page in (StartPage, PageOne, PageTwo,Window):

            frame = page(frame_container, self)

            self.frames[page] = frame

            frame.grid(row=0, column=0, sticky="nsew")

        self.topFrame(StartPage)

    def topFrame(self, cont):

        frame = self.frames[cont]
        frame.tkraise()

        
class StartPage(tk.Frame):

    def __init__(self, parent, controller):
        tk.Frame.__init__(self,parent)
        label = tk.Label(self, text="Load Graph", font=LARGE_FONT)
        label.pack(pady=10,padx=10)

        load_graph = tk.Button(self, text="Visit Page 1",
                            command=lambda: controller.show_frame(Window))
        load_graph.pack()


        



class Window(tk.Frame):
    def __init__(self, parent, controller):
        tk.Frame.__init__(self, parent)
        # self.parent = parent
        # self.pack(fill='both', expand=1)
        
        load = Image.open("ansuz.png")
        render = ImageTk.PhotoImage(load)
        img = tk.Label(self, image=render)
        img.image = render
        img.place(relwidth = 1, relheight = 1)

        tala_label = tk.Label(self, text = "TALA", bg = "#3e51c1",font = 50)
        tala_label.place(relx = .5, rely = 0.05, relwidth = .25, relheight = .1,anchor = 'n')
        
        create_label = tk.Label(self, text = "Create new Host", bg = "white")
        create_label.place(relx = .5, rely = .45, relwidth = .3, relheight = .05 ,anchor = 'n')

        create_bot = tk.Button(self, text = "Create")
        create_bot.place(relx = .5, rely = .5, relwidth = .2, relheight = .05,anchor = 'n')

        connect_label = tk.Label(self, text = "IP or DNS of Host", bg = "white")
        connect_label.place(relx = .5, rely = .65, relwidth = .3, relheight = .05 ,anchor = 'n')

        host_entry = tk.Entry(self, bg = 'white')
        host_entry.place(relx = .5, rely = .7, relwidth = .2, relheight = .05,anchor = 'n')

        connect_bot = tk.Button(self, text = "Connect", command=lambda:myframe())
        connect_bot.place(relx = .5, rely = .75, relwidth = .2, relheight = .05,anchor = 'n')
        

app = ManageFrames()
app.mainloop()
