import prata as tl
import time
import json
print("TEST: Testing for Statistics")
m = tl.connect("127.0.0.1", 25565)

m.host()
time.sleep(1)

sub = m.subscriber()
pub = m.publisher()

pub.publish("1","message1")
pub.publish("1","message2")

print("receiving 2 messages from channel 1")
if("message1" != sub.listen("1")):
    print("---TEST: failure in statistics listen uno")

if("message2" != sub.listen("1")):
    print("---TEST: failure in statistics listen dos")


i = 0
while i < 501:
    pub.publish("2",str(i))
    i+=1


i = 1
while i < 300:
    tmp = sub.listen("2")
    if(str(i) != tmp):
        print("---TEST: failure in mass recieve", str(i), tmp)
    i+=1

jsondata = str(json.loads(m.serialize()))
if(len(jsondata) < 20):
    print("---TEST: serialize failure")

m.terminate()

exit()
