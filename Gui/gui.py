import tkinter as tk

# pip install pillow
from PIL import Image, ImageTk

class Window(tk.Frame):
    def __init__(self, master=None):
        tk.Frame.__init__(self, master)
        self.master = master
        self.pack(fill='both', expand=1)
        
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
        
root = tk.Tk()
app = Window(root)
root.wm_title("Tkinter window")
root.geometry("800x700")
root.mainloop()