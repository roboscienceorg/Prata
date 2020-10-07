import numpy as np
import tkinter as tk
from graph import *


class Buttons():

    def __init__(self, parent, controller):
        self.publishers = {}
        self.subscribers = {}
        self.buttons()

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