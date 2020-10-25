import tkinter as tk
from tkinter import messagebox
from .TALA import Master, connect
import json
from .listbox import *
from .resizingcanvas import *





class Buttons():
    def __init__(self,parent):
        self.parent = parent

    def displayChannel(self, name):
        list = MultiListbox(self.parent, ['Name','IP', 'Port'], width = 10,highlightthickness=0, border=0)
        data = []
        print(name)
        print(self.parent.channels[name])
        data.append(name)
        data.append(self.parent.channels[name][0][0])
        data.append(self.parent.channels[name][0][1])


        list.add_data(data)
        list.place(relx = 0, rely = 1, anchor = 'sw')

    def displayPublishers(self,port):
        list = MultiListbox(self.parent, ['IP', 'Port'], width = 15,highlightthickness=0, border=0)
        data = []

        data.append(self.parent.publishers[port][0])
        data.append(port)


        list.add_data(data)
        list.place(relx = 0, rely = 1, anchor = 'sw')

    def displaySubscribers(self,port):
        list = MultiListbox(self.parent, ['IP', 'Port'], width = 15,highlightthickness=0, border=0)
        data = []

        data.append(self.parent.subscribers[port][0])
        data.append(port)

        list.add_data(data)
        list.place(relx = 0, rely = 1, anchor = 'sw')


    def leftButtons(self):
        left_button_canvas = ResizingCanvas(self.parent,width=850, height=400, bg="#7a7f85", highlightthickness=0)
        left_button_canvas.place(relx = 0, rely = 0, relwidth = .1, relheight = 1,anchor = 'nw')

        list_sub_bot = tk.Button(left_button_canvas, text = "Terminate",command=lambda: self.terminate())
        list_sub_bot.place(x = 0, rely = .1, relwidth = 1, relheight = .05,anchor = 'w')

        list_pub_bot = tk.Button(left_button_canvas, text = "List Publishers",command=lambda: self.listPublishers())
        list_pub_bot.place(x = 0, rely = .3, relwidth = 1, relheight = .05,anchor = 'w')

        list_chan_bot = tk.Button(left_button_canvas, text = "List Channels",command=lambda: self.listChannel())
        list_chan_bot.place(x = 0, rely = .5, relwidth = 1, relheight = .05,anchor = 'w')

        list_sub_bot = tk.Button(left_button_canvas, text = "List Subscribers",command=lambda: self.listSubscribers())
        list_sub_bot.place(x = 0, rely = .7, relwidth = 1, relheight = .05,anchor = 'w')

    def terminate(self):
        self.parent.connection.master.terminate()
        exit()

    def rightButtons(self):
        remove = tk.StringVar()
        right_button_canvas = ResizingCanvas(self.parent,width=850, height=400, bg="#7a7f85", highlightthickness=0)
        right_button_canvas.place(relx = 1, rely = 1, relwidth = .1, relheight = 1,anchor = 'se')

        remove_entry = tk.Entry(right_button_canvas, bg = 'white', textvariable = remove)
        remove_entry.place(relx = 1, rely = .1, relwidth = 1, relheight = .05,anchor = 'e',)

        delete_chan_bot = tk.Button(right_button_canvas, text = "Remove Channel", command=lambda: self.removeChan(remove))
        delete_chan_bot.place(relx = 1, rely = .7, relwidth = 1, relheight = .05,anchor = 'e',)

        refresh_bot = tk.Button(right_button_canvas, text = "Refresh", command=lambda: self.parent.refresh())
        refresh_bot.place(relx = 1, rely = .9, relwidth = 1, relheight = .05,anchor = 'e',)

    def listChannel(self):
        list = MultiListbox(self.parent, ['Name','IP', 'Port'], width = 10,highlightthickness=0, border=0)
        data = []

        print(self.parent.channels)
        for key in self.parent.channels:
            data.append(key)
            print(key)
            data.append(self.parent.channels[key][0][0])
            data.append(self.parent.channels[key][0][1])


        list.add_data(data)
        list.place(relx = 0, rely = 1, anchor = 'sw')

    def listPublishers(self):
        list = MultiListbox(self.parent, ['IP', 'Port'], width = 15,highlightthickness=0, border=0)
        data = []

        for key in self.parent.publishers:
            data.append(self.parent.publishers[key][0])
            data.append(key)


        list.add_data(data)
        list.place(relx = 0, rely = 1, anchor = 'sw')

    def listSubscribers(self):
        list = MultiListbox(self.parent, ['IP', 'Port'], width = 15,highlightthickness=0, border=0)
        data = []

        for key in self.parent.subscribers:
            data.append(self.parent.subscribers[key][0])
            data.append(key)

        list.add_data(data)
        list.place(relx = 0, rely = 1, anchor = 'sw')

    def removeChan(self,remove):
        channel = str(remove.get())
        try:
            self.connection.master.removeChannel(channel)
            self.refresh()
        except:
            tk.messagebox.showerror("Error", "Invalid Channel")