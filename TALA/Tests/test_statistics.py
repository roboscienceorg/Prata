import TALA as tl
import time
import json

m = tl.connect("127.0.0.1", 25565)

m.host()
print("setting up host...")
time.sleep(1)

sub = m.subscriber()
pub = m.publisher()

print("sending two messages to channel 1")
pub.publish("1","message1")
pub.publish("1","message2")

print("receiving 2 messages from channel 1")
uno = sub.listen("1")
dos = sub.listen("1")

print("sending 501 messages to channel 2")
i = 0
while i < 501:
    pub.publish("2",str(i))
    i+=1

print("receiving 300 messages from channel 2")
i = 0
while i < 300:
    sub.listen("2")
    i+=1
    
jsondata = json.loads(m.serialize())
print(jsondata["channels"]["1"]["channelStatistics"])
print(jsondata["channels"]["2"]["channelStatistics"])


#tl.gui()

m.terminate()

exit()
