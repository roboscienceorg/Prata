import tkinter as tk
from tkinter import messagebox
from .TALA import Master, connect
import json
from .listbox import *
from .resizingcanvas import *





class Buttons():
    def __init__(self,parent):
        self.parent = parent

    def createChanWin(self):
        chan_port = tk.StringVar()
        chan_name = tk.StringVar()
        chan_style = tk.StringVar()
        chan_limit = tk.StringVar()
        new_window = tk.Toplevel() 
  
        # sets the title of the 
        # Toplevel widget 
        new_window.title("Create Subscriber") 
      
        # sets the geometry of toplevel 
        new_window.geometry("400x400") 

        port_label = tk.Label(new_window, text = "Enter Channel port")
        port_label.place(relx = .25, rely = .2, relwidth = .5,  relheight = .1 ,anchor = 'center')

        port_entry = tk.Entry(new_window, bg = 'white', textvariable = chan_port)
        port_entry.place(relx = .25, rely = .3, relwidth = .4, relheight = .1,anchor = 'center')

        style_entry = tk.OptionMenu(new_window, chan_style, "FIFO", "BROADCAST")
        style_entry.place(relx = .75, rely = .3, relwidth = .4, relheight = .1,anchor = 'center')

        name_label = tk.Label(new_window, text = "Enter Channel name")
        name_label.place(relx = .25, rely = .4, relwidth = .5,  relheight = .1 ,anchor = 'center')

        name_entry = tk.Entry(new_window, bg = 'white', textvariable = chan_name)
        name_entry.place(relx = .25, rely = .5, relwidth = .4, relheight = .1,anchor = 'center')

        limit_label = tk.Label(new_window, text = "Enter Channel limit")
        limit_label.place(relx = .75, rely = .4, relwidth = .5,  relheight = .1 ,anchor = 'center')

        limit_entry = tk.Entry(new_window, bg = 'white', textvariable = chan_limit)
        limit_entry.place(relx = .75, rely = .5, relwidth = .4, relheight = .1,anchor = 'center')

        delete_chan_bot = tk.Button(new_window, text = "Create Channel", command=lambda: self.createChannel(chan_port,chan_name,chan_style,chan_limit,new_window))
        delete_chan_bot.place(relx = .5, rely = .8, relwidth = .5, relheight = .2,anchor = 'center')

    def createChannel(self,chan_port,chan_name,chan_style,chan_limit,new_window):
        port = int(chan_port.get())
        name = str(chan_name.get())
        style = str(chan_style.get())
        limit = int(chan_limit.get())

        try:
            chan = self.parent.connection.master.createChannel(port,name,style,limit)
            self.parent.refresh()
            new_window.destroy()
        except:
            tk.messagebox.showerror("Error", "Please enter channel")

    def createSubWin(self):
        channel_name = tk.StringVar()
        new_window = tk.Toplevel() 
  
        # sets the title of the 
        # Toplevel widget 
        new_window.title("Create Subscriber") 
      
        # sets the geometry of toplevel 
        new_window.geometry("200x200") 

        port_label = tk.Label(new_window, text = "Enter Channel name to \nconnect Subscriber to ")
        port_label.place(relx = .5, rely = .2, relwidth = 1,  relheight = .2 ,anchor = 'center')

        remove_entry = tk.Entry(new_window, bg = 'white', textvariable = channel_name)
        remove_entry.place(relx = .5, rely = .4, relwidth = .4, relheight = .2,anchor = 'center')

        delete_chan_bot = tk.Button(new_window, text = "Create Subscriber", command=lambda: self.createSubscriber(channel_name,new_window))
        delete_chan_bot.place(relx = .5, rely = .6, relwidth = .5, relheight = .2,anchor = 'center')

    def createSubscriber(self,channel_name,new_window):
        channel = str(channel_name.get())
        channel = channel.split(",")
        print(channel)
        try:
            sub = self.parent.connection.master.subscriber()
            for chan in channel:
                sub.connect(chan)
            self.parent.refresh()
            new_window.destroy()
        except:
            tk.messagebox.showerror("Error", "Please enter channel")

    def createPubWin(self):
        channel_name = tk.StringVar()
        new_window = tk.Toplevel() 
  
        # sets the title of the 
        # Toplevel widget 
        new_window.title("Create Publisher") 
      
        # sets the geometry of toplevel 
        new_window.geometry("200x200") 

        port_label = tk.Label(new_window, text = "Enter Channel name to \nconnect Publisher to ")
        port_label.place(relx = .5, rely = .2, relwidth = 1,  relheight = .2 ,anchor = 'center')

        remove_entry = tk.Entry(new_window, bg = 'white', textvariable = channel_name)
        remove_entry.place(relx = .5, rely = .4, relwidth = .4, relheight = .2,anchor = 'center')

        delete_chan_bot = tk.Button(new_window, text = "Create Publisher", command=lambda: self.createPublisher(channel_name,new_window))
        delete_chan_bot.place(relx = .5, rely = .6, relwidth = .5, relheight = .2,anchor = 'center')

    def createPublisher(self,channel_name,new_window):
        channel = str(channel_name.get())
        channel = channel.split(",")
        print(channel)
        try:
            pub = self.parent.connection.master.publisher()
            for chan in channel:
                pub.connect(chan)
            self.parent.refresh()
            new_window.destroy()
        except:
            tk.messagebox.showerror("Error", "Please enter channel")


    # displayChannel(self, name)
    # Takes in the name of a channel and displays that channels information to the use. 
    def displayChannel(self, name):
        list = MultiListbox(self.parent, ['Name','IP', 'Port'], width = 10,highlightthickness=0, border=0)
        data = []

        data.append(name)
        data.append(self.parent.channels[name][0][0])
        data.append(self.parent.channels[name][0][1])


        list.add_data(data)
        list.place(relx = 0, rely = 1, anchor = 'sw')

    # displayPublishers(self, port)
    # Takes in the port of a publisher and displays that publisher information to the use. 
    def displayPublishers(self,port):
        list = MultiListbox(self.parent, ['IP', 'Port'], width = 15,highlightthickness=0, border=0)
        data = []

        data.append(self.parent.publishers[port][0])
        data.append(port)


        list.add_data(data)
        list.place(relx = 0, rely = 1, anchor = 'sw')

    # displaySubscribers(self, port)
    # Takes in the port of a subscriber and displays that subscriber information to the use. 
    def displaySubscribers(self,port):
        list = MultiListbox(self.parent, ['IP', 'Port'], width = 15,highlightthickness=0, border=0)
        data = []

        data.append(self.parent.subscribers[port][0])
        data.append(port)

        list.add_data(data)
        list.place(relx = 0, rely = 1, anchor = 'sw')

    # leftButtons(self)
    # Creates and displays all the buttons on the left side of the screen. 
    # These buttons are:
    # Terminate, List Channels, List Publishers, List Subscribers
    def leftButtons(self):
        self.min_port = tk.StringVar()
        self.max_port = tk.StringVar()

        port_range = "Custom Port range: \n" + str(self.parent.connection.custom_range)
        if( self.parent.connection.custom_range == True):
            port_range += str(self.parent.connection.port_range)

        left_button_canvas = ResizingCanvas(self.parent,width=850, height=400, bg="#7a7f85", highlightthickness=0)
        left_button_canvas.place(relx = 0, rely = 0, relwidth = .1, relheight = 1,anchor = 'nw')

        custom_range_label = tk.Label(left_button_canvas, text = port_range)
        custom_range_label.place(relx = 1, rely = .1, relwidth = 1, relheight = .05,anchor = 'e')

        min_label = tk.Label(left_button_canvas, text = "Min")
        min_label.place(relx = .5, rely = .15, relwidth = .5, relheight = .05,anchor = 'e')

        max_label = tk.Label(left_button_canvas, text = "Max")
        max_label.place(relx = 1, rely = .15, relwidth = .5, relheight = .05,anchor = 'e')

        min_port_scale = tk.Scale(left_button_canvas, from_=1024 , to = 49151,orient="horizontal",showvalue=1)
        min_port_scale.place(relx = .5, rely = .2, relwidth = .5, relheight = .05,anchor = 'e')

        max_port_scale = tk.Scale(left_button_canvas, from_=1024 , to = 49151,orient="horizontal",showvalue=1)
        max_port_scale.place(relx = 1, rely = .2, relwidth = .5, relheight = .05,anchor = 'e')

        min_port_entry = tk.Entry(left_button_canvas, bg = 'white', textvariable = self.min_port )
        min_port_entry.place(relx = .5, rely = .25, relwidth = .5, relheight = .05,anchor = 'e')

        max_port_entry = tk.Entry(left_button_canvas, bg = 'white', textvariable = self.max_port )
        max_port_entry.place(relx = 1, rely = .25, relwidth = .5, relheight = .05,anchor = 'e')

        set_port_bot = tk.Button(left_button_canvas, text = "Set Range", command=lambda: [self.setPortRange(min_port_scale,max_port_scale)])
        set_port_bot.place(relx = 1, rely = .3, relwidth = 1, relheight = .05,anchor = 'e')

        create_chan_bot = tk.Button(left_button_canvas, text = "Add Channel", command=lambda: self.createChanWin())
        create_chan_bot.place(relx = 1, rely = .4, relwidth = 1, relheight = .05,anchor = 'e')

        create_pub_bot = tk.Button(left_button_canvas, text = "Add Publisher", command=lambda: self.createPubWin())
        create_pub_bot.place(relx = 1, rely = .5, relwidth = 1, relheight = .05,anchor = 'e')

        create_sub_bot = tk.Button(left_button_canvas, text = "Add Subscriber", command=lambda: self.createSubWin())
        create_sub_bot.place(relx = 1, rely = .6, relwidth = 1, relheight = .05,anchor = 'e')

        delete_chan_bot = tk.Button(left_button_canvas, text = "Remove Channel", command=lambda: self.removeChanWindow())
        delete_chan_bot.place(relx = 1, rely = .7, relwidth = 1, relheight = .05,anchor = 'e')


  
    # listChannel(self)  
    # Displays all channels information in a list box on the bottom left side of the screen
    def listChannel(self):
        list = MultiListbox(self.parent, ['Name','IP', 'Port'], width = 10,highlightthickness=0, border=0)
        data = []

        for key in self.parent.channels:
            data.append(key)
            data.append(self.parent.channels[key][0][0])
            data.append(self.parent.channels[key][0][1])


        list.add_data(data)
        list.place(relx = 1, rely = .2, anchor = 'e')

    # listPublishers(self)  
    # Displays all publsihers information in a list box on the bottom left side of the screen
    def listPublishers(self):
        list = MultiListbox(self.parent, ['IP', 'Port'], width = 15,highlightthickness=0, border=0)
        data = []

        for key in self.parent.publishers:
            data.append(self.parent.publishers[key][0])
            data.append(key)


        list.add_data(data)
        list.place(relx = 1, rely = .2, anchor = 'e')

    # listSubscribers(self)  
    # Displays all subscribers information in a list box on the bottom left side of the screen
    def listSubscribers(self):
        list = MultiListbox(self.parent, ['IP', 'Port'], width = 15,highlightthickness=0, border=0)
        data = []

        for key in self.parent.subscribers:
            data.append(self.parent.subscribers[key][0])
            data.append(key)

        list.add_data(data)
        list.place(relx = 1, rely = .2, anchor = 'e')

    # removeChan(self,remove)
    # Takes in the channel name that the use wishes to remove for the network.
    # The function trys to remove the given channel. If it fails it will alert the user
    # that the channel they tried to delete is an invalid channel.
    def removeChanWindow(self):
        remove = tk.StringVar()
        new_window = tk.Toplevel() 
  
        # sets the title of the 
        # Toplevel widget 
        new_window.title("Remove Channel") 
      
        # sets the geometry of toplevel 
        new_window.geometry("200x200") 

        port_label = tk.Label(new_window, text = "Enter Channel name to be removed")
        port_label.place(relx = .5, rely = .2, relwidth = 1,  relheight = .2 ,anchor = 'center')

        remove_entry = tk.Entry(new_window, bg = 'white', textvariable = remove)
        remove_entry.place(relx = .5, rely = .4, relwidth = .4, relheight = .2,anchor = 'center')

        delete_chan_bot = tk.Button(new_window, text = "Remove Channel", command=lambda: self.removeChan(remove,new_window))
        delete_chan_bot.place(relx = .5, rely = .6, relwidth = .5, relheight = .2,anchor = 'center')

    def removeChan(self,remove,new_window):
        channel = str(remove.get())
        try:
            self.parent.connection.master.removeChannel(channel)
            self.parent.refresh()
            new_window.destroy()
        except:
            tk.messagebox.showerror("Error", "Invalid Channel")

    # rightButtons(self)
    # Creates and displays all the buttons on the right side of the screen. 
    # There is an entry box for the user to fill in which channel they wish to be delete.
    # As far as buttons go there are Remove Channel and Refresh buttons.
    def rightButtons(self):
        right_button_canvas = ResizingCanvas(self.parent,width=850, height=400, bg="#7a7f85", highlightthickness=0)
        right_button_canvas.place(relx = 1, rely = 1, relwidth = .1, relheight = 1,anchor = 'se')
        
        list_chan_bot = tk.Button(right_button_canvas, text = "List Channels",command=lambda: self.listChannel())
        list_chan_bot.place(relx = 1, rely = .4, relwidth = 1, relheight = .05,anchor = 'e')

        list_pub_bot = tk.Button(right_button_canvas, text = "List Publishers",command=lambda: self.listPublishers())
        list_pub_bot.place(relx = 1, rely = .5, relwidth = 1, relheight = .05,anchor = 'e')

        list_sub_bot = tk.Button(right_button_canvas, text = "List Subscribers",command=lambda: self.listSubscribers())
        list_sub_bot.place(relx = 1, rely = .6, relwidth = 1, relheight = .05,anchor = 'e')

        refresh_bot = tk.Button(right_button_canvas, text = "Refresh", command=lambda: self.parent.refresh())
        refresh_bot.place(relx = 1, rely = .7, relwidth = 1, relheight = .05,anchor = 'e')

        terminate_bot = tk.Button(right_button_canvas, text = "Terminate",command=lambda: self.terminate())
        terminate_bot.place(relx = 1, rely = .8, relwidth = 1, relheight = .05,anchor = 'e')

        disconnect_bot = tk.Button(right_button_canvas, text = "Disconnect", command=lambda: self.parent.refresh())
        disconnect_bot.place(relx = 1, rely = .9, relwidth = 1, relheight = .05,anchor = 'e')
       


    def setPortRange(self,min_port,max_port):
        min = self.min_port.get()
        max = self.max_port.get()
        if min != "":
            min_port.set(min)
        if max != "":
            max_port.set(max)
        min = min_port.get()
        max = max_port.get()


        self.parent.connection.master.setPortRanges(min,max)

    # terminate(self)
    # Terminates the currently connected master process
    def terminate(self):
        self.parent.connection.master.terminate()
        exit()

