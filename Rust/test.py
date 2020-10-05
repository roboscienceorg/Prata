import TALA as tl
import time

m = tl.connect("127.0.0.1", 25565);

m.host(True);

time.sleep(5)


sub = m.subscriber()
pub = m.publisher()

pub.connect("test")
pub.publish("test","testing message 1=======")


sub.connect("test")
time.sleep(1)



print("listen 1 ", sub.listen("test"))

pub.publish("test","testing message2 ==========")

time.sleep(1)

print("listen 2 ", sub.listen("test"))
print("listen 3 ", sub.listen("test"))

m.terminate()

exit()
