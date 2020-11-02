import TALA as tl
import time
import json

m = tl.connect("192.168.0.122", 25565)

m.host()




time.sleep(1)


sub = m.subscriber()
pub = m.publisher()
pub1 = m.publisher()

pub.connect("test")
pub.connect("test1")

pub.publish("test","testing message 1=======")

sub.connect("test")
# sub.connect("test1")
# sub.connect("test3")

print("listen 1 ", sub.listen("test"))

pub.publish("test","testing message2 ==========")

print("listen 2 ", sub.listen("test"))
print("listen 3 ", sub.listen("test"))
pub.disconnect("test1")


tl.gui()

m.terminate()

exit()
