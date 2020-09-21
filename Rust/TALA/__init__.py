from .TALA import *
#from GUI import *

print("TALA Loaded!")

def masterConnect(IP):
    return masterConnectPy(IP)

def masterDisconnect(IP):
    return masterConnectPy(IP)

def subscribe(subscriber, channelName):
    return subscriberPy(subscriber, channelName)
