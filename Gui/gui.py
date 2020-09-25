import tkinter as tk
from graph import *

HEIGHT = 700
WIDTH = 800
root = tk.Tk()


canvas = tk.Canvas(root, height = HEIGHT, width = WIDTH)
canvas.pack()

background_image = tk.PhotoImage(file = "ansuz.png")
background_label = tk.Label(root, image = background_image)
background_label.place(relwidth = 1, relheight = 1)

tala_label = tk.Label(root, text = "TALA", bg = "#3e51c1",font = 50)
tala_label.place(relx = .5, rely = 0.05, relwidth = .25, relheight = .1,anchor = 'n')

create_label = tk.Label(root, text = "Create new Host", bg = "white")
create_label.place(relx = .5, rely = .45, relwidth = .3, relheight = .05 ,anchor = 'n')

create_bot = tk.Button(root, text = "Create")
create_bot.place(relx = .5, rely = .5, relwidth = .2, relheight = .05,anchor = 'n')

connect_label = tk.Label(root, text = "IP or DNS of Host", bg = "white")
connect_label.place(relx = .5, rely = .65, relwidth = .3, relheight = .05 ,anchor = 'n')

host_entry = tk.Entry(root, bg = 'white')
host_entry.place(relx = .5, rely = .7, relwidth = .2, relheight = .05,anchor = 'n')

connect_bot = tk.Button(root, text = "Connect")
connect_bot.place(relx = .5, rely = .75, relwidth = .2, relheight = .05,anchor = 'n')


root.mainloop()