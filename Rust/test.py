import TALA as tl
import time

m = tl.connect("127.0.0.1", 25565)

s = m.subscriber()
p = m.publisher()

#m.host(False)

time.sleep(1)




print("p connect")
p.connect("test")
print("p connected")

m.terminate()

exit()

print("s connect")
s.connect("test")
print("s connected")

p.publish("test", "I fucked your mom")
s.listen("test")



#exit()
