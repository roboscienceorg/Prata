from datetime import timedelta
import json
import matplotlib.pyplot as plt
import networkx as nx
import numpy as np
import os
from PIL import ImageTk,Image  
import socket
import tkinter as tk
from tkinter import messagebox
import time
from .prata import Master, connect
from .resizingcanvas import *
from .listbox import *


LARGE_FONT= ("Verdana", 20)

class ConnectionData():
    def __init__(self,parent):
        self.parent = parent
        self.channels = {}
        self.json_data = {}
        self.master_ip = ""
        self.master_port = 0
        self.master = {}
        self.port_range = [0, 0]
        self.next_port = 0
        self.custom_range = False
        self.connected = False
        self.created_pubs = {}
        self.created_subs = {}

    def clear(self):
        self.channels = {}
        self.json_data = {}
        self.master_ip = ""
        self.master_port = 0
        self.master = {}
        self.port_range = [0, 0]
        self.next_port = 0
        self.custom_range = False
        self.created_pubs = {}
        self.created_subs = {}

    # parseJson(self)
    # Takes the json from master and converts it into a form for use in the GUI
    def parseJson(self):

        cords = [0,0]
        publishers = []
        subscribers = []
        self.port_range = self.json_data["portRange"]
        self.custom_range = self.json_data["isCustomRange"]
        self.next_port = self.json_data["nextPort"]

        self.channels = self.json_data["channels"]

        return self.channels

    # connectMaster(self)
    # Using the ip and port from the connection screen tries to connect to a given master        
    def connectMaster(self,ip,port,new_window):
        self.master_ip = str(ip.get())
        self.master_port = int(port.get())
        self.master = connect(self.master_ip,int(self.master_port))
        self.parent.menu.menubar.entryconfig("Edit", state="normal")
        self.parent.menu.fileMenu.entryconfig("Disconnect", state="normal")
        new_window.destroy()
        self.connected = True
        self.parent.startGraph()

    def createMaster(self,new_port,new_window):
        self.master_ip = socket.gethostbyname(socket.gethostname())
        self.master_port = int(new_port.get())
        # try:
        self.master = connect(str(self.master_ip), self.master_port )
        self.master.host()
        new_window.destroy()
        self.parent.menu.menubar.entryconfig("Edit", state="normal")
        self.parent.menu.fileMenu.entryconfig("Disconnect", state="normal")
        self.connected = True

        self.parent.startGraph() 

        # except:
        #     tk.messagebox.showerror("Error", "Invalid Port entered \nPlease Re-enter and try again")

    # createChannel(self,chan_port,chan_name,chan_style,chan_limit,new_window)
    #     Takes in the new port, name, style, and limit on the channel to be create as well
    #     as a reference to the window that was create. This function trys to create a new channel
    #     with the given information and if it can it will refresh the page and destroy the create
    #     channel window. If it cannot it will display an error
    def createChannel(self,chan_port,chan_name,chan_style,chan_limit,new_window):
        port = 0
        try:
            port = int(chan_port.get())
        except:
            port = self.next_port
        name = str(chan_name.get())
        try:
            style = str(chan_style.get())
        except:
            style = "FIFO"
        try:
            limit = int(chan_limit.get())
        except:
            limit = 10

        try:

            new_window.destroy()        
            chan = self.master.createChannel(port,name,style,limit)
            self.parent.startGraph()

        except:
            tk.messagebox.showerror("Error", "Please enter correct channel information")
    

    # createPublisher(self,channel_name,new_window)
    #     Takes in a list of channel names and the reference to the new window.
    #     The function will split the list of channel names on a comma and then try and 
    #     create a publisher and connect it to each channel. If it does this correctly
    #     the window will be destroyed and the GUI will be refreshed. Otherwise it will
    #     display an error
    def createPublisher(self,channel_name,new_window):
        channel = str(channel_name.get())
        channel = channel.split(",")
        # try:
        pub = self.master.publisher()
        pub_port = str(pub.getPort())

        self.created_pubs[pub_port] = pub

        for chan in channel:
            pub.connect(str(chan))
        new_window.destroy()
        self.parent.startGraph()

        # except:
        #     tk.messagebox.showerror("Error", "Please enter channels")

    # createSubscriber(self,channel_name,new_window)
    #     Takes in a list of channel names and the reference to the new window.
    #     The function will split the list of channel names on a comma and then try and 
    #     create a subscriber and connect it to each channel. If it does this correctly
    #     the window will be destroyed and the GUI will be refreshed. Otherwise it will
    #     display an error
    def createSubscriber(self,channel_name,new_window):
        channel = str(channel_name.get())
        channel = channel.split(",")
        new_window.destroy()

        try:
            sub = self.master.subscriber()
            sub_port = str(sub.getPort())

            self.created_subs[sub_port] = sub

            for chan in channel:
                sub.connect(str(chan))

            self.parent.startGraph()
        except:
            tk.messagebox.showerror("Error", "Please enter channel")

    # deleteChan(self,remove,new_window)
    #     Takes in the channel to remove and a reference to the new window.
    #     If the function is able to remove the channel it will refresh the GUI and
    #     destroy the new window. Otherwise it will display an error
    def deleteChan(self,remove,new_window):
        channel = str(remove.get())
        try:
            self.master.removeChannel(channel)
            self.parent.startGraph()
            new_window.destroy()
        except:
            tk.messagebox.showerror("Error", "Invalid Channel")

    def getMessage(self,port,chan,window):

        try:
            window.destroy()
            new_window = tk.Toplevel() 

            sub_port = str(port.get())
            channel = chan.get()
            sub = self.created_subs[sub_port]
            message = sub.listen(channel)

            message_label = tk.Label(new_window, text = message)
            message_label.place(relx = .5, rely = .5, relwidth = 1,  relheight = 1 ,anchor = 'center')
            self.parent.startGraph()
        except:

            error_window = tk.Toplevel() 
            error_window.geometry("200x200") 
            invalid_label = tk.Label(error_window, text = "Invalid Subscriber")
            invalid_label.place(relx = .5, rely = .5, relwidth = 1,  relheight = 1 ,anchor = 'center')

    def sendMessage(self,port,chan,mess,window):
        # try:
        window.destroy()
        
        pub_port = str(port.get())
        channel = chan.get()
        message = mess.get()
        pub = self.created_pubs[pub_port]
        pub.publish(channel,message)
        self.parent.startGraph()
        # except:
        #     new_window = tk.Toplevel() 
        #     new_window.geometry("200x200") 
        #     invalid_label = tk.Label(new_window, text = "Invalid Publisher")
        #     invalid_label.place(relx = .5, rely = .5, relwidth = 1,  relheight = 1 ,anchor = 'center')

    # setPortRange(self,min_port,max_port)
    #     Takes in a min and max port. It checks first to see if the user entered a value into the boxes. 
    #     If so it sets the scale to those values and calls the function to set the port ranges for master.
    #     If no values were entered the it checks the scales values and uses those.
    def setPortRange(self,min_port,max_port,window):
        window.destroy()

        min = int(min_port.get())
        max = int(max_port.get())

        self.master.setPortRanges(min,max)
        self.parent.startGraph()


    # checkChan(self,channel)
    #   Takes in a channels name. It then walks through to see if any publisher has talked to the channel
    #   and if so when the last time was. It then repeats this process for all subscribers currently connected
    #   to that channel. If either a publisher or subscriber has talked to the given channel with the last
    #   minute the funcion will return True otherwise it will return false
    def checkChan(self,channel):
        last_used = timedelta.max
        current_time = timedelta(seconds = time.time())
        for pubs in self.channels[channel]["channelStatistics"]["pubTimestamps"]:
            try:
                if(timedelta(milliseconds = self.channels[channel]["channelStatistics"]["pubTimestamps"][pubs]) < last_used):
                    last_used = timedelta(milliseconds = self.channels[channel]["channelStatistics"]["pubTimestamps"][pubs])
            except:
                pass

        for subs in self.channels[channel]["channelStatistics"]["subTimestamps"]:
            try:
                if(timedelta(milliseconds = self.channels[channel]["channelStatistics"]["subTimestamps"][subs]) < last_used):
                    last_used = timedelta(milliseconds = self.channels[channel]["channelStatistics"]["subTimestamps"][subs])
            except:
                pass

        if(last_used != timedelta.max and current_time - last_used < timedelta(minutes = 1)):
            return True
        return False

    # checkPub(self,lookup)
    #     Takes in a publisher ip and port combination. It then uses this info to look
    #     in the channels dictionary to see if and when that publisher last talked to a 
    #     channel. If it has talked to it within the last  minute the function returns True
    #     otherwise it will return False
    def checkPub(self,lookup):
        last_used = timedelta.max
        current_time = timedelta(seconds = time.time())
        for name in self.channels:
            try:
                if(timedelta(milliseconds = self.channels[name]["channelStatistics"]["pubTimestamps"][lookup]) < last_used):
                    last_used = timedelta(milliseconds = self.channels[name]["channelStatistics"]["pubTimestamps"][lookup])
            except:
                pass

        if(last_used != timedelta.max and current_time - last_used < timedelta(minutes = 1)):
            return True
        return False

    # checkSub(self,lookup)
    #     Takes in a subscriber ip and port combination. It then uses this info to look
    #     in the channels dictionary to see if and when that subscriber last talked to a 
    #     channel. If it has talked to it within the last  minute the function returns True
    #     otherwise it will return False
    def checkSub(self,lookup):
        last_used = timedelta.max
        current_time = timedelta(seconds = time.time())
        for name in self.channels:
            try:
                if(timedelta(milliseconds = self.channels[name]["channelStatistics"]["subTimestamps"][lookup]) < last_used):
                    last_used = timedelta(milliseconds = self.channels[name]["channelStatistics"]["subTimestamps"][lookup])
            except:
                pass

        if(last_used != timedelta.max and current_time - last_used < timedelta(minutes = 1)):
            return True
        return False

    # terminate(self)
    # Terminates the currently connected master process
    def terminate(self):
        self.master.terminate()
        exit()

    # terminate(self)
    # Terminates the currently connected master process
    def exit(self):
        exit()

    # retrieveData(self)
    # This function calls the serialize function to get the current networks information.
    # It then loads this as json to be converted into a dictionary
    def retrieveData(self):
        self.json_data = json.loads(self.master.serialize())

class Graph(tk.Frame):
    def __init__(self, parent):
        tk.Frame.__init__(self, parent)
        self.parent = parent
    def startpage(self):
        self.canvas = ResizingCanvas(self,width=850,bg = "white", height=400, highlightthickness=0)
        self.canvas.pack(fill="both", expand=True)
        self.g = nx.DiGraph()
        self.channels = {}
        self.title()

    def showMasterInfo(self):
        master_font = ("Verdana", 12)
        ip = "Master IP: " + str(self.parent.host.master_ip)
        port = "Master Port: " + str(self.parent.host.master_port)

        create_label = tk.Label(self.canvas, text = ip,bg = "white",font = master_font)
        create_label.place(relx = 1, rely = 0 ,anchor = 'ne')
        create_label = tk.Label(self.canvas, text = port,bg = "white",font = master_font)
        create_label.place(relx = 1, rely = .04 ,anchor = 'ne')
        print(self.parent.host.custom_range)
        if(self.parent.host.custom_range == True):
            port_range = "Port Range: " + str(self.parent.host.port_range)
            create_label = tk.Label(self.canvas, text = port_range,bg = "white",font = master_font)
            create_label.place(relx = 1, rely = .08 ,anchor = 'ne')

    def clear(self):
        self.canvas = ResizingCanvas(self,width=850,bg = "white", height=400, highlightthickness=0)
        self.canvas.pack(fill="both", expand=True)
        self.g = nx.DiGraph()
        self.channels = {}
        self.title()

    def title(self):
        small_font= ("Verdana", 10)
        title = tk.Label(self.canvas, text = "Prata Network Management Tool",bg = "white",font = LARGE_FONT)
        title.place(relx = .5, rely = 0 ,anchor = 'n')
        # if(self.parent.host.connected == False):
        create_label = tk.Label(self.canvas, text = "Select File to join a network. Select Edit to begin interactions",bg = "white",font = small_font)
        create_label.place(relx = .5, rely = .04 ,anchor = 'n')

    def buildGraph(self):
        self.publishers = []  
        self.subscribers = []
        pubs_list = []  
        subs_list = []
        font_size = 8
        max = 1
        y_offset = 0
        new_nodetoplot = []
        old_nodetoplot = []
        good_chantoplot = []
        old_chantoplot = []
        edgestoplot = []

        self.g.clear()
        size_of_node = 266.66666
        self.g.add_node("Publishers",layer = 0)
        self.g.add_node("Channels",layer = 1,node_color = "white")
        self.g.add_node("Subscribers",layer = 2,node_color = "white")
        self.g.add_edge("Publishers","Channels")
        self.g.add_edge("Channels","Subscribers")

        label_list = {}
        width = self.canvas.width
        height = self.canvas.height

        for chan in self.channels:
            self.g.add_node(str(chan),layer = 1)

            for pubs in self.channels[chan]["publishers"]:
                ip_port = "IP:"+str(pubs[0]) + "\nPort:" + str(pubs[1])
                self.g.add_node(str(ip_port),layer = 0)
                self.g.add_edge(str(ip_port),str(chan))
                edgestoplot.append([str(ip_port),str(chan)])

                if pubs not in self.g:
                    self.publishers.append(pubs)
                    pubs_list.append(str(ip_port))


            for subs in self.channels[chan]["subscribers"]:
                ip_port = "IP:"+str(subs[0]) + "\nPort:" + str(subs[1])

                self.g.add_node(str(ip_port),layer = 2)
                self.g.add_edge(str(chan),str(ip_port))
                edgestoplot.append([str(chan),str(ip_port)])

                if subs not in self.g:
                    self.subscribers.append(subs)
                    subs_list.append(str(ip_port))

        if (len(self.channels) > 1):
            max = len(self.channels)
        if(len( self.subscribers) > max):
            max = len( self.subscribers)
        if(len( self.publishers) > max):
            max = len( self.publishers)

        plt.clf()
        pos = nx.multipartite_layout(self.g, subset_key="layer")
        nx.draw_networkx_nodes(self.g, pos, nodelist=["Publishers","Channels","Subscribers"], node_color="w")

        if ((1.0/max) * height*.1) > 8:
            font_size = 8
        else:
            font_size = ((1.0/max) * height*.1)

        if max < 3:
            y_offset = ((1.0/4)/2)
        else:
            y_offset = ((1.0/max)/2)

        for node in pos:
            x,y=pos[node]
            text = str(node)
            if(text != "Publishers" and text != "Channels" and text != "Subscribers"):
                if(text in pubs_list):
                    plt.text(x,y+y_offset,text,horizontalalignment='right',fontsize = font_size)
                    lookup = node.replace("IP:", "")
                    lookup = lookup.replace("\nPort", "")
                    if(self.parent.host.checkPub(lookup)):
                        new_nodetoplot.append(str(node))
                    else:
                        old_nodetoplot.append(str(node))
                if(text in subs_list):
                    plt.text(x,y+y_offset,text,horizontalalignment='left',fontsize = font_size)
                    lookup = node.replace("IP:", "")
                    lookup = lookup.replace("\nPort", "")
                    if(self.parent.host.checkSub(lookup)):
                        new_nodetoplot.append(str(node))
                    else:
                        old_nodetoplot.append(str(node))
                if(text in self.channels):
                    plt.text(x,y+y_offset,text,horizontalalignment='center',fontsize = font_size)
                    if(self.parent.host.checkChan(node)):
                        good_chantoplot.append(str(node))
                    else:
                        old_chantoplot.append(str(node))
            else:
                if(max == 0):
                    plt.text(x,0.0,text,horizontalalignment='center',fontsize = 8)
                elif(max ==1 ):
                    plt.text(x,0.4166666666666666666666,text,horizontalalignment='center',fontsize = 8)
                elif(max ==2 ):
                    plt.text(x,0.7777777777777777777777,text,horizontalalignment='center',fontsize = 8) 
                else:
                    plt.text(x,1.0,text,horizontalalignment='center',fontsize = 8)        


        if(((1.0/max) * height) < size_of_node):
           size_of_node = ((1.0/max) * height) 

        nx.draw_networkx_nodes(self.g,pos, nodelist=new_nodetoplot,node_size = size_of_node)
        nx.draw_networkx_nodes(self.g,pos, nodelist=old_nodetoplot,node_size = size_of_node, node_color="grey")

        nx.draw_networkx_nodes(self.g,pos, nodelist=good_chantoplot,node_size = size_of_node, node_shape='s')
        nx.draw_networkx_nodes(self.g,pos, nodelist=old_chantoplot,node_size = size_of_node, node_shape='s', node_color="grey")

        nx.draw_networkx_edges(self.g,pos, edgelist =edgestoplot,arrowstyle = "-")



        plt.axis('off')

        plt.savefig('figure.png')

        image = Image.open('figure.png')


        img_width, img_height = image. size
        rel_width = img_width / width
        rel_height = img_height / height

        image = image.resize((int((rel_width+.4)*width), int((rel_height+.4)*height)), Image.ANTIALIAS) ## The (250, 250) is (height, width)
        img = ImageTk.PhotoImage(image) 
        self.parent.one = img  
        self.canvas.create_image(width/2,height/2 , anchor='center', image=img)
        image.close()

        os.remove('figure.png')

    # listChannel(self)  
    # Displays all channels information in a list box on the bottom left side of the screen
    def listChannel(self):
        list = MultiListbox(self.canvas, ['Name','IP', 'Port'], width = 10,highlightthickness=0, border=0)
        data = []

        for key in self.channels:
            data.append(key)
            data.append(self.channels[key]["info"][0])
            data.append(self.channels[key]["info"][1])


        list.add_data(data)
        list.place(relx = 1, rely = 1,relwidth = .17,relheight = .15, anchor = 'se')

    # listPublishers(self)  
    # Displays all publsihers information in a list box on the bottom left side of the screen
    def listPublishers(self):
        list = MultiListbox(self.canvas, ['IP', 'Port'], width = 15,highlightthickness=0, border=0)
        data = []
        for key in range(len(self.publishers)):
            data.append(self.publishers[key][0])
            data.append(self.publishers[key][1])


        list.add_data(data)
        list.place(relx = 1, rely = 1,relwidth = .17,relheight = .15, anchor = 'se')

    # listSubscribers(self)  
    # Displays all subscribers information in a list box on the bottom left side of the screen
    def listSubscribers(self):
        list = MultiListbox(self.canvas, ['IP', 'Port'], width = 15,highlightthickness=0, border=0)
        data = []
        for key in range(len(self.subscribers)):
            data.append(self.subscribers[key][0])
            data.append(self.subscribers[key][1])

        list.add_data(data)
        list.place(relx = 1, rely = 1,relwidth = .17,relheight = .15, anchor = 'se')