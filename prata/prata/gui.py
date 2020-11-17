from multiprocessing import Process
from os.path import split
from PIL import Image, ImageTk
import tkinter as tk
import tkinter as tk
from tkinter import messagebox
from .graph import *
from .resizingcanvas import *
from .prata import Master, connect

class ManageFrames(tk.Tk):

    def __init__(self, *args, **kwargs):
        tk.Tk.__init__(self, *args, **kwargs)
        tk.Tk.wm_title(self, "Prata")
        self.geometry("1300x800")


        self.host = ConnectionData(self)



        self.graph_frame = Graph(self)
        self.graph_frame.startpage()
        self.graph_frame.pack(fill="both", expand=True)


        self.menu = filemenu(self)
        self.menu.startMenu()


    def disconnect(self):
        self.host.clear()

        self.graph_frame.canvas.destroy()
        self.graph_frame.startpage()
        self.graph_frame.pack(fill="both", expand=True)

        self.menu = filemenu(self)
        self.menu.startMenu()

    def startGraph(self):
        self.host.retrieveData()
        channel = self.host.parseJson()
        self.graph_frame.title()   
        self.graph_frame.showMasterInfo()
        self.graph_frame.pack(fill="both", expand=True)
     
        self.graph_frame.channels = channel
        self.graph_frame.buildGraph()


class filemenu(tk.Frame):

    def __init__(self, parent):
        tk.Frame.__init__(self, parent)
        self.parent = parent

    def startMenu(self):
        self.menubar = tk.Menu(tearoff=False)
        self.fileMenu = tk.Menu(tearoff=False)
        self.editMenu = tk.Menu(tearoff=False)

        self.menubar.add_cascade(label="File", menu=self.fileMenu)
        self.menubar.add_cascade(label="Edit", menu=self.editMenu ,state="disabled")

        self.fileMenu.add_command(label="Host", command=self.createMasterWin)
        self.fileMenu.add_command(label="Connect", command=self.connectMasterWin)
        self.fileMenu.add_command(label="Disconnect", command=self.parent.disconnect)
        self.fileMenu.add_command(label="Exit", command=self.parent.host.exit)
        self.fileMenu.entryconfig("Disconnect", state="disabled")


        channel = tk.Menu(self.editMenu,tearoff=False)
        channel.add_command(label="New", command=self.createChanWin)
        channel.add_command(label="Delete", command=self.deleteChanWindow)
        channel.add_command(label="Show", command=self.parent.graph_frame.listChannel)
        channel.add_command(label="Set Port Range", command=self.setPortRangeChanWindow)


        publisher = tk.Menu(self.editMenu,tearoff=False)
        publisher.add_command(label="Create", command=self.createPubWin)
        publisher.add_command(label="Send", command=self.sendMessWindow)
        publisher.add_command(label="Show", command=self.parent.graph_frame.listPublishers)

        subscriber = tk.Menu(self.editMenu,tearoff=False)
        subscriber.add_command(label="Create", command=self.createSubWin)
        subscriber.add_command(label="Listen", command=self.getMessageWin)
        subscriber.add_command(label="Show", command=self.parent.graph_frame.listSubscribers)

        self.editMenu.add_cascade(label='Channel', menu=channel, underline=0)
        self.editMenu.add_cascade(label='Publisher', menu=publisher, underline=0)
        self.editMenu.add_cascade(label='Subscriber', menu=subscriber, underline=0)

        self.parent.configure(menu=self.menubar)

    def createMasterWin(self):
        master_port = tk.StringVar()
        new_window = tk.Toplevel() 
  
        # sets the title of the 
        # Toplevel widget 
        new_window.title("Create Master") 
      
        # sets the geometry of toplevel 
        new_window.geometry("150x100") 

        port_label = tk.Label(new_window, text = "Enter Port of new Master")
        port_label.place(relx = .5, rely = .3, relwidth = 1,  relheight = .2 ,anchor = 'center')

        chan_entry = tk.Entry(new_window, bg = 'white', textvariable = master_port)
        chan_entry.place(relx = .5, rely = .5, relwidth = .5, relheight = .2,anchor = 'center')

        create_sub_bot = tk.Button(new_window, text = "Create Master",\
         command=lambda: self.parent.host.createMaster(master_port,new_window))
        create_sub_bot.place(relx = .5, rely = .75, relwidth = .5, relheight = .2,anchor = 'center')

    def connectMasterWin(self):
        master_port = tk.StringVar()
        master_ip = tk.StringVar()

        new_window = tk.Toplevel() 
  
        # sets the title of the 
        # Toplevel widget 
        new_window.title("Create Master") 
      
        # sets the geometry of toplevel 
        new_window.geometry("200x200") 


        ip_label = tk.Label(new_window, text = "Enter IP\nof Master")
        ip_label.place(relx = .25, rely = .2, relwidth = .4,  relheight = .2 ,anchor = 'center')

        ip_entry = tk.Entry(new_window, bg = 'white', textvariable = master_ip)
        ip_entry.place(relx = .25, rely = .4, relwidth = .4, relheight = .2,anchor = 'center')
        
        port_label = tk.Label(new_window, text = "Enter Port\nof Master")
        port_label.place(relx = .75, rely = .2, relwidth = .4,  relheight = .2 ,anchor = 'center')

        port_entry = tk.Entry(new_window, bg = 'white', textvariable = master_port)
        port_entry.place(relx = .75, rely = .4, relwidth = .4, relheight = .2,anchor = 'center')

        create_sub_bot = tk.Button(new_window, text = "Connect to Master", \
            command=lambda: self.parent.host.connectMaster(master_ip,master_port,new_window))
        create_sub_bot.place(relx = .5, rely = .6, relwidth = .5, relheight = .2,anchor = 'center')

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

        create_chan_bot = tk.Button(new_window, text = "Create Channel", \
            command=lambda: self.parent.host.createChannel(chan_port,chan_name,chan_style,chan_limit,new_window))
        create_chan_bot.place(relx = .5, rely = .8, relwidth = .5, relheight = .2,anchor = 'center')

    def deleteChanWindow(self):
        remove = tk.StringVar()
        new_window = tk.Toplevel() 
  
        # sets the title of the 
        # Toplevel widget 
        new_window.title("Remove Channel") 
      
        # sets the geometry of toplevel 
        new_window.geometry("150x150") 

        chan_label = tk.Label(new_window, text = "Enter Channel name \nto be removed")
        chan_label.place(relx = .5, rely = .3, relwidth = 1,  relheight = .2 ,anchor = 'center')

        chan_entry = tk.Entry(new_window, bg = 'white', textvariable = remove)
        chan_entry.place(relx = .5, rely = .5, relwidth = .4, relheight = .2,anchor = 'center')

        delete_chan_bot = tk.Button(new_window, text = "Remove Channel", command=lambda: self.parent.host.deleteChan(remove,new_window))
        delete_chan_bot.place(relx = .5, rely = .8, relwidth = .6, relheight = .2,anchor = 'center')

    def createPubWin(self):
        channel_name = tk.StringVar()
        new_window = tk.Toplevel() 
  
        # sets the title of the 
        # Toplevel widget 
        new_window.title("Create Publisher") 
      
        # sets the geometry of toplevel 
        new_window.geometry("200x200") 

        chan_label = tk.Label(new_window, text = "Enter Channel names to \nconnect Publisher to.")
        chan_label.place(relx = .5, rely = .2, relwidth = 1,  relheight = .2 ,anchor = 'center')

        chan_entry = tk.Entry(new_window, bg = 'white', textvariable = channel_name)
        chan_entry.place(relx = .5, rely = .4, relwidth = .4, relheight = .2,anchor = 'center')

        create_pub_bot = tk.Button(new_window, text = "Create Publisher",\
         command=lambda: self.parent.host.createPublisher(channel_name,new_window))
        create_pub_bot.place(relx = .5, rely = .6, relwidth = .5, relheight = .2,anchor = 'center')

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

        chan_entry = tk.Entry(new_window, bg = 'white', textvariable = channel_name)
        chan_entry.place(relx = .5, rely = .4, relwidth = .4, relheight = .2,anchor = 'center')

        create_sub_bot = tk.Button(new_window, text = "Create Subscriber",\
         command=lambda: self.parent.host.createSubscriber(channel_name,new_window))
        create_sub_bot.place(relx = .5, rely = .6, relwidth = .5, relheight = .2,anchor = 'center')

    def sendMessWindow(self):
        channel_name = tk.StringVar()
        pub_port = tk.StringVar()
        message = tk.StringVar()     
        new_window = tk.Toplevel() 
  
        # sets the title of the 
        # Toplevel widget 
        new_window.title("Send message") 
      
        # sets the geometry of toplevel 
        new_window.geometry("200x200") 

        chan_label = tk.Label(new_window, text = "Channel name \nto publish to")
        chan_label.place(relx = .25, rely = .1, relwidth = .5,  relheight = .2 ,anchor = 'center')

        chan_entry = tk.Entry(new_window, bg = 'white', textvariable = channel_name)
        chan_entry.place(relx = .25, rely = .3, relwidth = .4, relheight = .2,anchor = 'center')

        port_label = tk.Label(new_window, text = "Enter port \nof publisher")
        port_label.place(relx = .75, rely = .1, relwidth = .5,  relheight = .2 ,anchor = 'center')

        port_entry = tk.Entry(new_window, bg = 'white', textvariable = pub_port)
        port_entry.place(relx = .75, rely = .3, relwidth = .4, relheight = .2,anchor = 'center')

        message_label = tk.Label(new_window, text = "Enter Message")
        message_label.place(relx = .5, rely = .5, relwidth = 1,  relheight = .2 ,anchor = 'center')

        message_entry = tk.Entry(new_window, bg = 'white', textvariable = message)
        message_entry.place(relx = .5, rely = .7, relwidth = .4, relheight = .2,anchor = 'center')

        send_message_bot = tk.Button(new_window, text = "Send message", command=lambda:self.parent.host.sendMessage(pub_port,channel_name,message,new_window))
        send_message_bot.place(relx = .5, rely = .9, relwidth = .5, relheight = .2,anchor = 'center')

    def getMessageWin(self):
        channel_name = tk.StringVar()
        sub_port = tk.StringVar()
        new_window = tk.Toplevel() 
  
        # sets the title of the 
        # Toplevel widget 
        new_window.title("Get message") 
      
        # sets the geometry of toplevel 
        new_window.geometry("200x200") 

        chan_label = tk.Label(new_window, text = "Channel name \nto listen to")
        chan_label.place(relx = .25, rely = .1, relwidth = .5,  relheight = .2 ,anchor = 'center')

        chan_entry = tk.Entry(new_window, bg = 'white', textvariable = channel_name)
        chan_entry.place(relx = .25, rely = .3, relwidth = .4, relheight = .2,anchor = 'center')

        port_label = tk.Label(new_window, text = "Enter port \nof subscriber")
        port_label.place(relx = .75, rely = .1, relwidth = .5,  relheight = .2 ,anchor = 'center')

        port_entry = tk.Entry(new_window, bg = 'white', textvariable = sub_port)
        port_entry.place(relx = .75, rely = .3, relwidth = .4, relheight = .2,anchor = 'center')

        get_message_bot = tk.Button(new_window, text = "Get message", \
            command=lambda: self.parent.host.getMessage(sub_port,channel_name,new_window))
        get_message_bot.place(relx = .5, rely = .9, relwidth = .5, relheight = .2,anchor = 'center')

    def destroyWin(self,window):
        window.destroy()

    def setPortRangeChanWindow(self):
        min_port = tk.StringVar()
        max_port = tk.StringVar()
        new_window = tk.Toplevel() 
  
        # sets the title of the 
        # Toplevel widget 
        new_window.title("Set Port Range") 
      
        # sets the geometry of toplevel 
        new_window.geometry("200x200") 

        min_label = tk.Label(new_window, text = "Min")
        min_label.place(relx = .5, rely = .2, relwidth = .5, relheight = .2,anchor = 'e')

        max_label = tk.Label(new_window, text = "Max")
        max_label.place(relx = 1, rely = .2, relwidth = .5, relheight = .2,anchor = 'e')

        min_port_entry = tk.Entry(new_window, bg = 'white', textvariable = min_port )
        min_port_entry.place(relx = .5, rely = .4, relwidth = .5, relheight = .2,anchor = 'e')

        max_port_entry = tk.Entry(new_window, bg = 'white', textvariable = max_port )
        max_port_entry.place(relx = 1, rely = .4, relwidth = .5, relheight = .2,anchor = 'e')

        set_port_bot = tk.Button(new_window, text = "Set Range", command=lambda: [self.parent.host.setPortRange(min_port,max_port,new_window)])
        set_port_bot.place(relx = 1, rely = .7, relwidth = 1, relheight = .2,anchor = 'e')

def gui():
    app = ManageFrames()
    app.mainloop()
