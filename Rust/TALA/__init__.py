from .TALA import *
from .gui import *
import json

print("TALA Loaded!")

def gg(m):

    gui(json.loads(m.serialize()))#st is a json of master_process object
