import TALA as tl
import time

m = tl.connect("127.0.0.1", 25565);

m.host(True);

time.sleep(1)


sub = m.subscriber()
pub = m.publisher()

pub.connect("test")
pub.publish("test","testing message 1=======")

sub.connect("test")
# sub.connect("test1")
# sub.connect("test3")

print("listen 1 ", sub.listen("test"))

pub.publish("test","testing message2 ==========")

print("listen 2 ", sub.listen("test"))
print("listen 3 ", sub.listen("test"))

tl.gui("127.0.0.1", 25565)


m.terminate()   

exit()
