import prata as tl
import time
import json

m = tl.connect("127.0.0.1", 25565)

m.host()
time.sleep(1)

sub = m.subscriber()
pub = m.publisher()

pub.publish("1","message1")
pub.publish("1","message2")

print("receiving 2 messages from channel 1")
if(uno != sub.listen("1"))
    print("---TEST: failure in statistics listen uno")

if(dos != sub.listen("1"))
    print("---TEST: failure in statistics listen dos")


i = 0
while i < 501:
    pub.publish("2",str(i))
    i+=1


i = 0
while i < 300:
    if(str(i) != sub.listen("2"))
        print("---TEST: failure in mass recieve")
    i+=1

jsondata = json.loads(m.serialize())
if(len(jsondata) < 20)
    print("---TEST: serialize failure")
#print(jsondata["channels"]["1"]["channelStatistics"])
#print(jsondata["channels"]["2"]["channelStatistics"])


#tl.gui()

m.terminate()

exit()
